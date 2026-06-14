#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, Symbol};

// ---------------------------------------------------------------------------
// HomeworkBounty
// ---------------------------------------------------------------------------
// A minimal Soroban contract that lets a teacher post a homework problem
// as a bounty, lets students submit a hashed solution, and lets the teacher
// pick a winner. No XLM is actually transferred — this contract is purely
// storage + authorization + bookkeeping focused.
// ---------------------------------------------------------------------------

#[contract]
pub struct HomeworkBounty;

#[contractimpl]
impl HomeworkBounty {
    // -----------------------------------------------------------------------
    // post_bounty
    // -----------------------------------------------------------------------
    /// Create a new homework bounty.
    ///
    /// The caller (`teacher`) becomes the owner of the bounty, declares a
    /// `problem_id` (a short `Symbol` identifier for the problem), and a
    /// `deadline` (a unix timestamp). Returns the newly assigned numeric
    /// `bounty_id`, which is unique per contract instance.
    pub fn post_bounty(
        env: Env,
        teacher: Address,
        problem_id: Symbol,
        deadline: u64,
    ) -> u32 {
        // The teacher must sign this call.
        teacher.require_auth();

        // Allocate a fresh bounty id (1, 2, 3, ...).
        let count: u32 = env
            .storage()
            .instance()
            .get(&"bounty_count")
            .unwrap_or(0u32);
        let bounty_id = count + 1;
        env.storage().instance().set(&"bounty_count", &bounty_id);

        // Initialise the per-bounty data slots. We store the fields as
        // individual entries keyed by the bounty id so that reads and writes
        // remain O(1) and don't require custom contracttype structs.
        env.storage()
            .instance()
            .set(&("teacher", bounty_id), &teacher);
        env.storage()
            .instance()
            .set(&("problem", bounty_id), &problem_id);
        env.storage()
            .instance()
            .set(&("deadline", bounty_id), &deadline);
        env.storage()
            .instance()
            .set(&("awarded", bounty_id), &false);
        env.storage()
            .instance()
            .set(&("submission_count", bounty_id), &0u32);

        // Empty submissions map: Address (student) -> Symbol (answer_hash).
        let submissions: Map<Address, Symbol> = Map::new(&env);
        env.storage()
            .instance()
            .set(&("submissions", bounty_id), &submissions);

        bounty_id
    }

    // -----------------------------------------------------------------------
    // submit_solution
    // -----------------------------------------------------------------------
    /// Submit a solution to a bounty.
    ///
    /// `student` signs the call, supplies `answer_hash` (a `Symbol` — e.g. the
    /// hex / base32 hash of the answer text) and the `bounty_id` they want to
    /// submit to. A student may only submit once per bounty and the teacher
    /// is not allowed to submit to their own bounty. Submissions after the
    /// deadline or after the bounty has been awarded are rejected.
    pub fn submit_solution(
        env: Env,
        student: Address,
        bounty_id: u32,
        answer_hash: Symbol,
    ) {
        // The student must sign this call.
        student.require_auth();

        // Make sure the bounty exists.
        let teacher: Address = env
            .storage()
            .instance()
            .get(&("teacher", bounty_id))
            .expect("Bounty not found");

        // The teacher cannot submit to their own bounty.
        if teacher == student {
            panic!("Teacher cannot submit to own bounty");
        }

        // No more submissions once the bounty is awarded.
        let awarded: bool = env
            .storage()
            .instance()
            .get(&("awarded", bounty_id))
            .unwrap_or(false);
        if awarded {
            panic!("Bounty already awarded");
        }

        // No more submissions once the deadline has passed.
        let deadline: u64 = env
            .storage()
            .instance()
            .get(&("deadline", bounty_id))
            .expect("Missing deadline");
        if env.ledger().timestamp() > deadline {
            panic!("Deadline passed");
        }

        // Reject duplicate submissions from the same student.
        let mut submissions: Map<Address, Symbol> = env
            .storage()
            .instance()
            .get(&("submissions", bounty_id))
            .unwrap_or(Map::new(&env));
        if submissions.get(student.clone()).is_some() {
            panic!("Student already submitted");
        }

        // Record the submission and bump the counter.
        submissions.set(student.clone(), answer_hash);
        env.storage()
            .instance()
            .set(&("submissions", bounty_id), &submissions);

        let count: u32 = env
            .storage()
            .instance()
            .get(&("submission_count", bounty_id))
            .unwrap_or(0u32);
        env.storage()
            .instance()
            .set(&("submission_count", bounty_id), &(count + 1));
    }

    // -----------------------------------------------------------------------
    // pick_winner
    // -----------------------------------------------------------------------
    /// The teacher picks a winner for a bounty.
    ///
    /// Only the original teacher that posted the bounty can call this. The
    /// `winner` must be an address that actually submitted a solution. The
    /// bounty is then marked as `awarded` so that no further submissions
    /// or re-awards are possible.
    pub fn pick_winner(
        env: Env,
        teacher: Address,
        bounty_id: u32,
        winner: Address,
    ) {
        // The teacher must sign this call.
        teacher.require_auth();

        // Make sure the bounty exists and the caller is its teacher.
        let stored_teacher: Address = env
            .storage()
            .instance()
            .get(&("teacher", bounty_id))
            .expect("Bounty not found");
        if teacher != stored_teacher {
            panic!("Only the teacher can pick a winner");
        }

        // Bounty can only be awarded once.
        let awarded: bool = env
            .storage()
            .instance()
            .get(&("awarded", bounty_id))
            .unwrap_or(false);
        if awarded {
            panic!("Bounty already awarded");
        }

        // The winner must have submitted a solution.
        let submissions: Map<Address, Symbol> = env
            .storage()
            .instance()
            .get(&("submissions", bounty_id))
            .unwrap_or(Map::new(&env));
        if submissions.get(winner.clone()).is_none() {
            panic!("Winner did not submit a solution");
        }

        // Record the winner and mark as awarded.
        env.storage()
            .instance()
            .set(&("winner", bounty_id), &winner);
        env.storage()
            .instance()
            .set(&("awarded", bounty_id), &true);
    }

    // -----------------------------------------------------------------------
    // submission_count (view)
    // -----------------------------------------------------------------------
    /// Return the number of distinct submissions recorded for `bounty_id`.
    /// Returns `0` if the bounty does not exist.
    pub fn submission_count(env: Env, bounty_id: u32) -> u32 {
        env.storage()
            .instance()
            .get(&("submission_count", bounty_id))
            .unwrap_or(0u32)
    }

    // -----------------------------------------------------------------------
    // get_winner (view)
    // -----------------------------------------------------------------------
    /// Return the recorded winner address for a bounty. If the bounty has
    /// not been awarded yet, the stored "winner" slot still holds the
    /// teacher's address (the initial placeholder) and `is_awarded` will
    /// be `false`. Panics if the bounty does not exist.
    pub fn get_winner(env: Env, bounty_id: u32) -> Address {
        env.storage()
            .instance()
            .get(&("winner", bounty_id))
            .expect("Bounty not found")
    }

    // -----------------------------------------------------------------------
    // is_awarded (view)
    // -----------------------------------------------------------------------
    /// Return whether a bounty has been awarded. Returns `false` if the
    /// bounty does not exist.
    pub fn is_awarded(env: Env, bounty_id: u32) -> bool {
        env.storage()
            .instance()
            .get(&("awarded", bounty_id))
            .unwrap_or(false)
    }
}

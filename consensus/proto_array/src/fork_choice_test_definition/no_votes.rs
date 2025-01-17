use types::FixedBytesExtended;

use super::*;

pub fn get_no_votes_test_definition() -> ForkChoiceTestDefinition {
    let balances = vec![0; 16];

    let operations = vec![
        // Check that the head is the finalized block.
        Operation::FindHead {
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances.clone(),
            expected_head: Hash256::zero(),
        },
        // Add block 2
        //
        //         0
        //        /
        //        2
        Operation::ProcessBlock {
            slot: Slot::new(1),
            root: get_root(2),
            parent_root: Hash256::zero(),
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
        },
        // Ensure the head is 2
        //
        //         0
        //        /
        //        2 <- head
        Operation::FindHead {
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances.clone(),
            expected_head: get_root(2),
        },
        // Add block 1
        //
        //         0
        //        / \
        //        2  1
        Operation::ProcessBlock {
            slot: Slot::new(1),
            root: get_root(1),
            parent_root: get_root(0),
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
        },
        // Ensure the head is still 2
        //
        //         0
        //        / \
        // head-> 2  1
        Operation::FindHead {
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances.clone(),
            expected_head: get_root(2),
        },
        // Add block 3
        //
        //         0
        //        / \
        //        2  1
        //           |
        //           3
        Operation::ProcessBlock {
            slot: Slot::new(2),
            root: get_root(3),
            parent_root: get_root(1),
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
        },
        // Ensure 2 is still the head
        //
        //          0
        //         / \
        // head-> 2  1
        //           |
        //           3
        Operation::FindHead {
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances.clone(),
            expected_head: get_root(2),
        },
        // Add block 4
        //
        //         0
        //        / \
        //        2  1
        //        |  |
        //        4  3
        Operation::ProcessBlock {
            slot: Slot::new(2),
            root: get_root(4),
            parent_root: get_root(2),
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
        },
        // Ensure the head is 4.
        //
        //         0
        //        / \
        //        2  1
        //        |  |
        // head-> 4  3
        Operation::FindHead {
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances.clone(),
            expected_head: get_root(4),
        },
        // Add block 5 with a justified epoch of 2
        //
        //         0
        //        / \
        //        2  1
        //        |  |
        //        4  3
        //        |
        //        5 <- justified epoch = 2
        Operation::ProcessBlock {
            slot: Slot::new(3),
            root: get_root(5),
            parent_root: get_root(4),
            justified_checkpoint: get_checkpoint(2),
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
        },
        // Ensure the head is now 5 whilst the justified epoch is 0.
        //
        //         0
        //        / \
        //        2  1
        //        |  |
        // head-> 4  3
        //        |
        //        5
        Operation::FindHead {
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances.clone(),
            expected_head: get_root(5),
        },
        // Ensure there is no error when starting from a block that has the
        // wrong justified epoch.
        //
        //      0
        //     / \
        //     2  1
        //     |  |
        //     4  3
        //     |
        //     5 <- starting from 5 with justified epoch 0 should error.
        //
        // Since https://github.com/ethereum/consensus-specs/pull/3431 it is valid
        // to elect head blocks that have a higher justified checkpoint than the
        // store.
        Operation::FindHead {
            justified_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: get_root(5),
            },
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances.clone(),
            expected_head: get_root(5),
        },
        // Set the justified epoch to 2 and the start block to 5 and ensure 5 is the head.
        //
        //      0
        //     / \
        //     2  1
        //     |  |
        //     4  3
        //     |
        //     5 <- head
        Operation::FindHead {
            justified_checkpoint: get_checkpoint(2),
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances.clone(),
            expected_head: get_root(5),
        },
        // Add block 6
        //
        //      0
        //     / \
        //     2  1
        //     |  |
        //     4  3
        //     |
        //     5
        //     |
        //     6
        Operation::ProcessBlock {
            slot: Slot::new(4),
            root: get_root(6),
            parent_root: get_root(5),
            justified_checkpoint: get_checkpoint(2),
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
        },
        // Ensure 6 is the head
        //
        //      0
        //     / \
        //     2  1
        //     |  |
        //     4  3
        //     |
        //     5
        //     |
        //     6 <- head
        Operation::FindHead {
            justified_checkpoint: get_checkpoint(2),
            finalized_checkpoint: Checkpoint {
                epoch: Epoch::new(1),
                root: Hash256::zero(),
            },
            justified_state_balances: balances,
            expected_head: get_root(6),
        },
    ];

    ForkChoiceTestDefinition {
        finalized_block_slot: Slot::new(0),
        justified_checkpoint: Checkpoint {
            epoch: Epoch::new(1),
            root: Hash256::zero(),
        },
        finalized_checkpoint: Checkpoint {
            epoch: Epoch::new(1),
            root: Hash256::zero(),
        },
        operations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let test = get_no_votes_test_definition();
        test.run();
    }
}

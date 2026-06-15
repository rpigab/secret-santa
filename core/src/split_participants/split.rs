/*
Because the search for Hamiltonian cycles is a NP-complete problem,
for graphs over 10 participants, it is wise to avoid trying to look for complete graphs,
but instead divide the graph into N/4 chunks of 4 participants (if N % 4 = 0).

When N % 4 is not zero, we want the last chunk, named the complement, to hold
not 1, 2 or 3 participants, but rather 5, 6 or 7.
 */

use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;

use crate::data::participants_data::ParticipantsData;

const CHUNK_SIZE: usize = 4;


/// Split the participants into independent subgroups (quatuors + a complement),
/// each carrying only the constraints whose members all belong to that subgroup.
///
/// Constraints that straddle two subgroups are trivially satisfied: members of
/// different subgroups can never be assigned to each other, so they are dropped.
pub(crate) fn split_participants(participants_data: ParticipantsData) -> Vec<ParticipantsData> {
    let ParticipantsData {
        mut participants,
        already_gifted_before,
        couples,
    } = participants_data;

    // Shuffle so constraints (couples, gifting history) get spread across the
    // subgroups instead of piling up on a single quatuor, which could leave one
    // person blocked from every group-mate and make a subgroup unsolvable.
    participants.shuffle(&mut rand::thread_rng());

    let (num_quatuors, size_complement) =
        calculate_split_number_of_participants(participants.len());

    let mut name_groups: Vec<Vec<String>> = Vec::new();

    // Create quatuors using chunks
    for chunk in participants.chunks(CHUNK_SIZE).take(num_quatuors) {
        name_groups.push(chunk.to_vec());
    }

    // Handle the complement (always at least CHUNK_SIZE participants)
    if size_complement > 0 {
        let complement_start = num_quatuors * CHUNK_SIZE;
        name_groups.push(participants[complement_start..].to_vec());
    }

    // Assert that we've used all participants exactly once
    assert_eq!(name_groups.iter().flatten().count(), participants.len());

    let groups: Vec<ParticipantsData> = name_groups.into_iter()
        .map(|group| build_group_data(group, &couples, &already_gifted_before))
        .collect();

    log::debug!("groups:\n{groups:#?}");

    groups
}

/// Build a [`ParticipantsData`] for a single subgroup, keeping only the
/// constraints whose members all belong to that subgroup.
fn build_group_data(
    group: Vec<String>,
    couples: &Option<Vec<(String, String)>>,
    already_gifted_before: &Option<HashMap<String, Vec<String>>>,
) -> ParticipantsData {
    let members: HashSet<&String> = group.iter().collect();

    let group_couples = couples.as_ref().map(|couples| {
        couples.iter()
            .filter(|(a, b)| members.contains(a) && members.contains(b))
            .cloned()
            .collect::<Vec<_>>()
    });

    let group_already_gifted_before = already_gifted_before.as_ref().map(|gifted| {
        gifted.iter()
            .filter(|(giver, _)| members.contains(giver))
            .map(|(giver, recipients)| {
                let recipients = recipients.iter()
                    .filter(|recipient| members.contains(recipient))
                    .cloned()
                    .collect::<Vec<_>>();
                (giver.clone(), recipients)
            })
            .collect::<HashMap<_, _>>()
    });

    ParticipantsData {
        participants: group,
        already_gifted_before: group_already_gifted_before,
        couples: group_couples,
    }
}


/// Calculate the number of participants to put in each subgraph
fn calculate_split_number_of_participants(n: usize) -> (usize, usize) {
    let num_quatuors = (n - CHUNK_SIZE) / CHUNK_SIZE;
    let size_complement = (n - CHUNK_SIZE) % CHUNK_SIZE + CHUNK_SIZE;

    (num_quatuors, size_complement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_chunks() {
        assert_eq!((1, 7), calculate_split_number_of_participants(11));
        assert_eq!((1, 4), calculate_split_number_of_participants(8));
    }

    #[test]
    fn splits_into_quatuors_and_complement() {
        let data = ParticipantsData {
            participants: (1..=13).map(|i| i.to_string()).collect(),
            already_gifted_before: None,
            couples: None,
        };

        let groups = split_participants(data);

        // 13 = 2 quatuors (4 + 4) + complement (5)
        let sizes: Vec<usize> = groups.iter().map(|g| g.participants.len()).collect();
        assert_eq!(sizes, vec![4, 4, 5]);
    }

    #[test]
    fn keeps_only_in_group_constraints() {
        // Force the same group membership regardless of the shuffle by making
        // the would-be straddling pair impossible to colocate: with 8 people in
        // groups of 4, members "a".."d" vs "e".."h" can land together in any mix,
        // so instead we assert invariants that hold for every possible split.
        let data = ParticipantsData {
            participants: (1..=8).map(|i| i.to_string()).collect(),
            already_gifted_before: Some(HashMap::from([
                ("1".to_string(), vec!["2".to_string(), "3".to_string()]),
            ])),
            couples: Some(vec![
                ("1".to_string(), "2".to_string()),
            ]),
        };

        let groups = split_participants(data);

        for group in &groups {
            let members: HashSet<&String> = group.participants.iter().collect();

            // every kept couple is fully inside the group
            for (a, b) in group.couples.as_ref().unwrap() {
                assert!(members.contains(a) && members.contains(b));
            }

            // every kept gifting entry, and its recipients, are inside the group
            for (giver, recipients) in group.already_gifted_before.as_ref().unwrap() {
                assert!(members.contains(giver));
                for recipient in recipients {
                    assert!(members.contains(recipient));
                }
            }
        }

        // the couple (1, 2) is kept iff 1 and 2 share a group, dropped otherwise
        let kept_couples: usize = groups.iter()
            .map(|g| g.couples.as_ref().unwrap().len())
            .sum();
        let one_two_together = groups.iter().any(|g| {
            let m: HashSet<&String> = g.participants.iter().collect();
            m.contains(&"1".to_string()) && m.contains(&"2".to_string())
        });
        assert_eq!(kept_couples, if one_two_together { 1 } else { 0 });
    }
}

/*
Because the search for Hamiltonian cycles is a NP-complete problem,
for graphs over 10 participants, it is wise to avoid trying to look for complete graphs,
but instead divide the graph into N/4 chunks of 4 participants (if N % 4 = 0).

When N % 4 is not zero, we want the last chunk, named the complement, to hold
not 1, 2 or 3 participants, but rather 5, 6 or 7.
 */

use std::collections::{HashMap, HashSet};

use crate::data::participants_data::ParticipantsData;

const CHUNK_SIZE: usize = 4;


/// Split the participants into independent subgroups (quatuors + a complement),
/// each carrying only the constraints whose members all belong to that subgroup.
///
/// Constraints that straddle two subgroups are trivially satisfied: members of
/// different subgroups can never be assigned to each other, so they are dropped.
pub(crate) fn split_participants(participants_data: ParticipantsData) -> Vec<ParticipantsData> {
    let ParticipantsData {
        participants,
        already_gifted_before,
        couples,
    } = participants_data;

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
        let data = ParticipantsData {
            participants: (1..=8).map(|i| i.to_string()).collect(),
            already_gifted_before: Some(HashMap::from([
                ("1".to_string(), vec!["2".to_string(), "5".to_string()]),
            ])),
            // (1, 3) are in the first group, (4, 8) straddle two groups
            couples: Some(vec![
                ("1".to_string(), "3".to_string()),
                ("4".to_string(), "8".to_string()),
            ]),
        };

        let groups = split_participants(data);

        // first group: participants 1..=4
        let first = &groups[0];
        assert_eq!(first.couples, Some(vec![("1".to_string(), "3".to_string())]));
        // "5" is dropped from already_gifted_before of "1", "2" is kept
        assert_eq!(
            first.already_gifted_before,
            Some(HashMap::from([("1".to_string(), vec!["2".to_string()])]))
        );

        // second group: participants 5..=8, the straddling couple is dropped
        let second = &groups[1];
        assert_eq!(second.couples, Some(vec![]));
    }
}

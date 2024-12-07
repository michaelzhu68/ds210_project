use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Member {
    id: usize,
    age: u32,
    gender: String,
    weight: f32,
    height: f32,
    max_bpm: u32,
    avg_bpm: u32,
    resting_bpm: u32,
    session_duration: f32,
    calories_burned: f32,
    workout_type: String,
    fat_percentage: f32,
    water_intake: f32,
    workout_frequency: u32,
    experience_level: u32,
    bmi: f32,
    connections: HashSet<usize>, 
}

impl Member {
    fn new(id: usize, fields: Vec<&str>) -> Self {
        Self {
            id,
            age: fields[0].parse().unwrap(),
            gender: fields[1].to_string(),
            weight: fields[2].parse().unwrap(),
            height: fields[3].parse().unwrap(),
            max_bpm: fields[4].parse().unwrap(),
            avg_bpm: fields[5].parse().unwrap(),
            resting_bpm: fields[6].parse().unwrap(),
            session_duration: fields[7].parse().unwrap(),
            calories_burned: fields[8].parse().unwrap(),
            workout_type: fields[9].to_string(),
            fat_percentage: fields[10].parse().unwrap(),
            water_intake: fields[11].parse().unwrap(),
            workout_frequency: fields[12].parse().unwrap(),
            experience_level: fields[13].parse().unwrap(),
            bmi: fields[14].parse().unwrap(),
            connections: HashSet::new(),
        }
    }
}

fn parse_csv(file_path: &str) -> Vec<Member> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut members = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        if idx == 0 {
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();
        members.push(Member::new(idx - 1, fields));
    }

    members
}

fn calculate_similarity(m1: &Member, m2: &Member) -> f32 {
    let mut score = 0.0;

    if m1.workout_type == m2.workout_type {
        score += 1.0;
    }

    let duration_diff = (m1.session_duration - m2.session_duration).abs();
    score += 1.0 / (1.0 + duration_diff);

    let frequency_diff = (m1.workout_frequency as i32 - m2.workout_frequency as i32).abs() as f32;
    score += 1.0 / (1.0 + frequency_diff);

    if m1.experience_level == m2.experience_level {
        score += 1.0;
    }

    let calories_diff = (m1.calories_burned - m2.calories_burned).abs();
    score += 1.0 / (1.0 + calories_diff / 1000.0); 

    let fat_diff = (m1.fat_percentage - m2.fat_percentage).abs();
    score += 1.0 / (1.0 + fat_diff);

    let bmi_diff = (m1.bmi - m2.bmi).abs();
    score += 1.0 / (1.0 + bmi_diff);

    score
}

fn find_best_buddies_and_degrees(
    members: &mut [Member],
    similarity_threshold: f32,
) -> HashMap<usize, usize> {
    let mut best_buddies = HashMap::new();

    for i in 0..members.len() {
        let mut best_match = None;
        let mut highest_score = 0.0;

        for j in 0..members.len() {
            if i == j {
                continue; 
            }

            let similarity = calculate_similarity(&members[i], &members[j]);

            if similarity > similarity_threshold {
                members[i].connections.insert(members[j].id);
                members[j].connections.insert(members[i].id);

                if similarity > highest_score {
                    highest_score = similarity;
                    best_match = Some(members[j].id);
                }
            }
        }

        if let Some(best_match_id) = best_match {
            best_buddies.insert(members[i].id, best_match_id);
        }
    }

    best_buddies
}

fn main() {
    let file_path = "/Users/michaelzhu/desktop/gym_members_exercise_tracking.csv";
    let mut members = parse_csv(file_path);

    let similarity_threshold = 2.0; 
    let best_buddies = find_best_buddies_and_degrees(&mut members, similarity_threshold);

    for member in &members {
        let best_buddy = best_buddies.get(&member.id).unwrap_or(&0);
        println!(
            "Member {}: Best buddy is Member {}, and has {} connections.",
            member.id,
            best_buddy,
            member.connections.len(),
        );
    }
}
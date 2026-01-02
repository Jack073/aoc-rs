use chrono::{NaiveDateTime, Timelike};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum LogType {
    BeginShift(usize),
    Asleep,
    Awaken,
}

struct LogLine {
    timestamp: NaiveDateTime,
    log_type: LogType,
}

impl From<String> for LogLine {
    fn from(value: String) -> Self {
        let split_line = value.split_at(19);

        let mut line_determinant = split_line.1.split_whitespace().take(2);
        LogLine {
            timestamp: NaiveDateTime::parse_from_str(split_line.0, "[%Y-%m-%d %H:%M] ").unwrap(),
            log_type: match line_determinant.next() {
                Some("Guard") => LogType::BeginShift(
                    line_determinant
                        .next()
                        .unwrap()
                        .trim_start_matches('#')
                        .parse()
                        .unwrap(),
                ),
                Some("falls") => LogType::Asleep,
                Some("wakes") => LogType::Awaken,
                _ => panic!("unexpected info in line"),
            },
        }
    }
}

fn part_one() -> usize {
    let mut logs = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.into())
        .collect::<Vec<LogLine>>();

    logs.sort_by_key(|log| log.timestamp);

    let mut guard_num = 0;

    logs.iter()
        .chunk_by(|g| match g.log_type {
            LogType::Awaken => guard_num,
            LogType::Asleep => guard_num,
            LogType::BeginShift(n) => {
                guard_num = n;
                n
            }
        })
        .into_iter()
        .sorted_by_key(|c| c.0)
        .into_iter()
        .map(|(id, entries)| {
            let mut data = [0; 60];
            let mut sleeping_from = None;

            entries.into_iter().for_each(|entry| match entry.log_type {
                LogType::BeginShift(_) => {}
                LogType::Asleep => {
                    sleeping_from = Some(entry.timestamp);
                }
                LogType::Awaken => {
                    for min in sleeping_from.take().unwrap().minute()..entry.timestamp.minute() {
                        data[min as usize] += 1;
                    }
                }
            });

            (id, data)
        })
        .sorted_by_key(|k| k.0)
        .chunk_by(|k| k.0)
        .into_iter()
        .map(|(key, data_group)| {
            (
                key,
                data_group.into_iter().fold([0; 60], |mut acc, e| {
                    for (i, elem) in e.1.iter().enumerate() {
                        acc[i] += *elem;
                    }

                    acc
                }),
            )
        })
        .max_by_key(|m| m.1.iter().sum::<usize>())
        .map(|(key, data)| {
            key * data
                .iter()
                .enumerate()
                .max_by_key(|&(_, b)| b)
                .map(|(a, _)| a)
                .unwrap()
        })
        .unwrap()
}

fn main() {
    println!("{}", part_one());
}

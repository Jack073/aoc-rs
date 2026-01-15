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

fn get_guard_data<'a, T>(f: T) -> usize
where
    T: Fn(Box<dyn Iterator<Item = (usize, [usize; 60])>>) -> usize,
{
    let mut logs = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.into())
        .collect::<Vec<LogLine>>();

    logs.sort_by_key(|log| log.timestamp);

    f(Box::new(
        logs.chunk_by(|a, b| {
            if let LogType::BeginShift(_) = b.log_type {
                false
            } else {
                true
            }
        })
        .into_iter()
        .map(|entries| {
            let mut data = [0; 60];
            let mut sleeping_from = None;
            let mut id = 0;

            entries.into_iter().for_each(|entry| match entry.log_type {
                LogType::BeginShift(g) => {
                    id = g;
                }
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
        .sorted_by_key(|&(k, _)| k)
        .into_iter()
        .chunk_by(|(k, _)| *k)
        .into_iter()
        .map(|(key, data_group)| {
            (
                key,
                data_group.into_iter().fold([0usize; 60], |mut acc, e| {
                    for (i, elem) in e.1.iter().enumerate() {
                        acc[i] += *elem;
                    }

                    acc
                }),
            )
        })
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter(),
    ))
}

fn part_one() -> usize {
    get_guard_data(|data| {
        data.max_by_key(|m| m.1.iter().sum::<usize>())
            .map(|(key, data)| {
                key * data
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, b)| b)
                    .map(|(a, _)| a)
                    .unwrap()
            })
            .unwrap()
    })
}

fn part_two() -> usize {
    struct GuardData {
        guard: usize,
        minute: usize,
        count: usize,
    }
    get_guard_data(|res| {
        res.map(|(id, data)| {
            let (minute, count) = data.iter().enumerate().max_by_key(|&(_, b)| b).unwrap();
            GuardData {
                guard: id,
                minute,
                count: *count,
            }
        })
        .max_by_key(|data| data.count)
        .map(|data| data.guard * data.minute)
        .unwrap()
    })
}

fn main() {
    println!("{}", part_two());
}

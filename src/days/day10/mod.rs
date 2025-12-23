use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use anyhow::Context;

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    indicator_light_diagram: Vec<bool>,
    button_wiring_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

fn press(light_diagram: &[bool], button: &[usize]) -> Vec<bool> {
    let mut updated_light_diagram = light_diagram.to_vec();

    for &light in button {
        updated_light_diagram[light] = !updated_light_diagram[light];
    }

    updated_light_diagram
}

fn is_correctly_configured(machine: &Machine, light_diagram: &[bool]) -> bool {
    machine.indicator_light_diagram.eq(light_diagram)
}

struct ButtonPressState {
    indicator_light_diagram: Vec<bool>,
    presses: usize,
}

impl Machine {
    fn min_button_presses_to_configure_indicator_lights(&self) -> usize {
        let mut queue = VecDeque::new();

        queue.push_front(ButtonPressState {
            indicator_light_diagram: vec![false; self.indicator_light_diagram.len()],
            presses: 0,
        });

        loop {
            let ButtonPressState {
                presses,
                indicator_light_diagram,
            } = queue.pop_front().unwrap();

            for button in &self.button_wiring_schematics {
                let updated_light_diagram = press(&indicator_light_diagram, button);

                if is_correctly_configured(self, &updated_light_diagram) {
                    return presses + 1;
                }

                queue.push_back(ButtonPressState {
                    indicator_light_diagram: updated_light_diagram,
                    presses: presses + 1,
                });
            }
        }
    }

    fn min_button_presses_to_configure_joltage_level_counters(&self) -> usize {
        let mut buttons_by_counter: Vec<(usize, Vec<usize>)> = (0..self.joltage_requirements.len())
            .map(|counter| {
                (
                    counter,
                    self.button_wiring_schematics
                        .iter()
                        .enumerate()
                        .filter(|(_, buttons)| buttons.contains(&counter))
                        .map(|(i, _)| i)
                        .collect(),
                )
            })
            .collect();

        let mut joltage = vec![0usize; self.joltage_requirements.len()];
        let mut presses = 0;

        // e.g. (0,1) (2) (0,2,3) {21,1,163,20}
        // 1. We know only (0,1) contains 1, so we _must_ have exactly 1 press of this button.
        // 2. We also know only (0,2,3) contains 3, so we _must_ have exactly 20 presses of this button
        // 3. This leaves 0 and 2
        // 4. With just (2) remaining, we should know _all_ remaining presses must be this button

        // e.g.2 (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7} -> 10
        // 1. 2 buttons make 0
        //      need 3x (0,2) and (0,1)
        //    2 buttons make 1
        //      need 5x (1,3) and (0,1)
        //    3 buttons make 2
        //      need 4x (2), (2,3) and (0,2)
        //    3 buttons make 3
        //      need 7x (3), (1,3) and (2,3)

        while let Some((i, buttons)) = buttons_by_counter
            .iter()
            .find(|(_, buttons)| buttons.len() == 1)
        {
            let remaining = self.joltage_requirements[*i] - joltage[*i];
            let button = buttons[0];
            presses += remaining;

            let counters = &self.button_wiring_schematics[button];

            for counter in counters {
                joltage[*counter] += remaining;
            }

            buttons_by_counter.iter_mut().for_each(|(_, buttons)| {
                if let Some(index) = buttons.iter().position(|value| *value == button) {
                    buttons.swap_remove(index);
                }
            });
        }

        // if self.joltage_requirements.iter().eq(&vec![21, 1, 163, 20]) {
        //     dbg!(buttons_by_counter);
        //     dbg!(&joltage);
        // }

        if joltage.iter().eq(&self.joltage_requirements) {
            return presses;
        }

        let remainder: Vec<usize> = joltage
            .iter()
            .zip(&self.joltage_requirements)
            .map(|(current, remainder)| remainder - current)
            .collect();

        // for (i, buttons) in buttons_by_counter {
        //     let requirement = self.joltage_requirements[i];

        //     if buttons.len() == 1 {
        //         let button = buttons[0];
        //         presses += requirement;

        //         let counters = &self.button_wiring_schematics[button];

        //         for counter in counters {
        //             joltage[*counter] += requirement;
        //         }

        //         // for _ in 0..buttons_by_counter.len() {
        //         //     let buttons = buttons_by_counter[button];

        //         //     if buttons.1.contains(&button) {
        //         //         buttons.1.remove(button);
        //         //     }
        //         // }
        //     } else {
        //         // make up `requirement` from any combination of `buttons`
        //         // e.g. max of button 1 is 10, max of button 2 is 5, requirement is 10,
        //         // could be 5x1 and 5x2, or 6x1 and 4x2 etc.

        //         // ideas:
        //         // 1. prioritise smaller requirements first
        //         // 2. prioritise using buttons affecting more counters

        //         // TODO reframe the problem as "X lots of button a, b, c, Y lots of button b,c d etc."

        //         let mut i = 0;

        //         let mut queue: VecDeque<(Vec<usize>, usize)> = VecDeque::new();
        //         let mut seen: HashSet<(Vec<usize>, usize)> = HashSet::new();

        //         queue.push_back((joltage.clone(), presses));
        //         seen.insert((joltage, presses));

        //         loop {
        //             let (joltage_level_counters, presses) = queue.pop_front().unwrap();

        //             i += 1;

        //             let max_presses_per_button: Vec<usize> = self
        //                 .button_wiring_schematics
        //                 .iter()
        //                 .map(|button| {
        //                     joltage_level_counters
        //                         .iter()
        //                         .enumerate()
        //                         .filter_map(|(i, &current)| {
        //                             if !button.contains(&i) {
        //                                 None
        //                             } else {
        //                                 let requirement = self.joltage_requirements[i];
        //                                 if requirement > current {
        //                                     Some(requirement - current)
        //                                 } else {
        //                                     None
        //                                 }
        //                             }
        //                         })
        //                         .min()
        //                         .unwrap_or_default()
        //                 })
        //                 .collect();

        //             for (_, button) in max_presses_per_button
        //                 .iter()
        //                 .zip(&self.button_wiring_schematics)
        //                 .filter(|(max, _)| **max != 0)
        //             {
        //                 let mut updated_joltage_levels = joltage_level_counters.clone();

        //                 for &counter in button {
        //                     updated_joltage_levels[counter] += 1;
        //                 }

        //                 if updated_joltage_levels.iter().eq(&self.joltage_requirements) {
        //                     dbg!(i);
        //                     return presses + 1;
        //                 }

        //                 if !seen.contains(&(updated_joltage_levels.clone(), presses + 1)) {
        //                     queue.push_back((updated_joltage_levels.clone(), presses + 1));
        //                     seen.insert((updated_joltage_levels, presses + 1));
        //                 }
        //             }
        //         }
        //     }
        // }

        1
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();

        let lights: &str = parts[0];

        let indicator_light_diagram = lights[1..lights.len() - 1]
            .chars()
            .map(|c| match c {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => anyhow::bail!("expected . or #"),
            })
            .collect::<Result<Vec<bool>, Self::Err>>()?;

        let button_wiring_schematics = parts[1..parts.len() - 1]
            .iter()
            .map(|&part| {
                part[1..part.len() - 1]
                    .split(",")
                    .map(|button| {
                        button
                            .parse::<usize>()
                            .context(format!("expected {} to be a digit", button))
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<usize>>, Self::Err>>()?;

        let joltage_requirements = parts[parts.len() - 1];
        let joltage_requirements = joltage_requirements[1..joltage_requirements.len() - 1]
            .split(",")
            .map(|s| str::parse::<usize>(s).context(format!("expected {} to be a usize digit", s)))
            .collect::<Result<Vec<usize>, Self::Err>>()?;

        Ok(Self {
            indicator_light_diagram,
            button_wiring_schematics,
            joltage_requirements,
        })
    }
}

fn parse(input: String) -> anyhow::Result<Vec<Machine>> {
    input
        .lines()
        .map(str::parse::<Machine>)
        .collect::<anyhow::Result<Vec<Machine>>>()
}

pub fn solve(input: String) -> anyhow::Result<()> {
    let machines = parse(input)?;

    // println!(
    //     "part1: {}",
    //     machines
    //         .iter()
    //         .map(|m| { m.min_button_presses_to_configure_indicator_lights() })
    //         .sum::<usize>()
    // );

    println!(
        "part2: {}",
        machines
            .iter()
            .map(|m| { m.min_button_presses_to_configure_joltage_level_counters() })
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_min_button_presses_to_configure_indicator_lights_1() {
        let machine = Machine {
            indicator_light_diagram: vec![false, true, true, false],
            button_wiring_schematics: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            joltage_requirements: vec![3, 5, 4, 7],
        };

        assert_eq!(
            machine.min_button_presses_to_configure_indicator_lights(),
            2
        );
    }

    #[test]
    fn test_machine_min_button_presses_to_configure_indicator_lights_2() {
        let machine = Machine {
            indicator_light_diagram: vec![false, false, false, true, false],
            button_wiring_schematics: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4],
            ],
            joltage_requirements: vec![7, 5, 12, 7, 2],
        };

        assert_eq!(
            machine.min_button_presses_to_configure_indicator_lights(),
            3
        );
    }

    #[test]
    fn test_machine_min_button_presses_to_configure_indicator_lights_3() {
        let machine = Machine {
            indicator_light_diagram: vec![false, true, true, true, false, true],
            button_wiring_schematics: vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 3, 4],
                vec![0, 1, 2, 4, 5],
                vec![1, 2],
            ],
            joltage_requirements: vec![10, 11, 11, 5, 10, 5],
        };

        assert_eq!(
            machine.min_button_presses_to_configure_indicator_lights(),
            2
        );
    }

    #[test]
    fn test_machine_min_button_presses_to_configure_joltage_level_counters() {
        let machine = Machine {
            indicator_light_diagram: vec![false, true, true, false],
            button_wiring_schematics: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            joltage_requirements: vec![3, 5, 4, 7],
        };

        assert_eq!(
            machine.min_button_presses_to_configure_joltage_level_counters(),
            10
        );
    }

    #[test]
    fn test_parse_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string();

        assert_eq!(
            parse(input).unwrap(),
            vec![Machine {
                indicator_light_diagram: vec![false, true, true, false],
                button_wiring_schematics: vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                joltage_requirements: vec![3, 5, 4, 7],
            }]
        )
    }
}

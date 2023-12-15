pub fn day7() {
    let str = include_str!("../day7.txt");

    let total = part2(str);
    println!("total: {}", total);
}

fn part1(str: &str) -> i32 {
    let mut input = parse(str);
    input.sort_unstable_by(|(a, _), (b, _)| {
        let (ta, tb) = (get_cards_type(a), get_cards_type(b));
        if ta == tb {
            for (ca, cb) in a.chars().zip(b.chars()) {
                if ca != cb {
                    return map_label(ca).cmp(&map_label(cb));
                }
            }
            std::cmp::Ordering::Equal
        } else {
            (ta as u32).cmp(&(tb as u32))
        }
    });
    input
        .iter()
        .enumerate()
        .map(|(i, (_, v))| (i + 1) as i32 * *v)
        .sum()
}

fn part2(str: &str) -> i32 {
    let mut input = parse(str);
    input.sort_unstable_by(|(a, _), (b, _)| {
        let (ta, tb) = (get_cards_type2(a), get_cards_type2(b));
        if ta == tb {
            for (ca, cb) in a.chars().zip(b.chars()) {
                if ca != cb {
                    return map_label2(ca).cmp(&map_label2(cb));
                }
            }
            std::cmp::Ordering::Equal
        } else {
            (ta as u32).cmp(&(tb as u32))
        }
    });
    input
        .iter()
        .enumerate()
        .map(|(i, (_, v))| (i + 1) as i32 * *v)
        .sum()
}

fn parse(str: &str) -> Vec<(&str, i32)> {
    str.lines()
        .map(|s| {
            let (a, b) = s.split_once(' ').unwrap();
            (a, b.trim().parse().unwrap())
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
enum CardsType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

// part1
fn map_label(c: char) -> usize {
    const LABEL: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    LABEL.iter().position(|&m| m == c).unwrap()
}

fn get_cards_type(cards: &str) -> CardsType {
    let mut counts = [0; 13];
    for c in cards.chars() {
        counts[map_label(c)] += 1;
    }
    counts.sort_unstable();
    match counts[8..] {
        [_, _, _, _, 5] => CardsType::FiveOfAKind,
        [_, _, _, _, 4] => CardsType::FourOfAKind,
        [_, _, _, 2, 3] => CardsType::FullHouse,
        [_, _, _, _, 3] => CardsType::ThreeOfAKind,
        [_, _, _, 2, 2] => CardsType::TwoPairs,
        [_, _, _, _, 2] => CardsType::OnePair,
        _ => CardsType::HighCard,
    }
}

// part2
fn map_label2(c: char) -> usize {
    const LABEL: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    LABEL.iter().position(|&m| m == c).unwrap()
}

fn get_cards_type2(cards: &str) -> CardsType {
    let mut counts = [0; 13];
    for c in cards.chars() {
        counts[map_label2(c)] += 1;
    }
    let j_count = counts[map_label2('J')];
    counts.sort_unstable();
    match counts[8..] {
        [_, _, _, _, 5] => CardsType::FiveOfAKind,
        [_, _, _, _, 4] if j_count == 1 || j_count == 4 => CardsType::FiveOfAKind,
        [_, _, _, _, 4] => CardsType::FourOfAKind,
        [_, _, _, 2, 3] if j_count == 2 || j_count == 3 => CardsType::FiveOfAKind,
        [_, _, _, 2, 3] => CardsType::FullHouse,
        [_, _, _, _, 3] if j_count == 1 || j_count == 3 => CardsType::FourOfAKind,
        [_, _, _, _, 3] => CardsType::ThreeOfAKind,
        [_, _, _, 2, 2] if j_count == 2 => CardsType::FourOfAKind,
        [_, _, _, 2, 2] if j_count == 1 => CardsType::FullHouse,
        [_, _, _, 2, 2] => CardsType::TwoPairs,
        [_, _, _, _, 2] if j_count == 1 || j_count == 2 => CardsType::ThreeOfAKind,
        [_, _, _, _, 2] => CardsType::OnePair,
        _ if j_count == 1 => CardsType::OnePair,
        _ => CardsType::HighCard,
    }
}

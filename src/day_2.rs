use regex::Regex;


pub fn day_2(verbose:bool)->String{
    let p1 = d_2_part_1(verbose);
    let p2 = d_2_part_2(verbose);
    let output:String = format!("Part 1:{}\nPart 2:{}",p1.to_string().yellow().bold(),p2.to_string().yellow().bold());
    return output;
}
fn d_2_part_1(verbose:bool) -> i128 {

    let mut score:i128=0;
    let samples = d_2_data();

    for sample in samples {
        let re = Regex::new(r"^(.*?)(-)(.*)$").unwrap();
        let lower_bound: i128 = re.captures(&sample).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let upper_bound: i128= re.captures(&sample).unwrap().get(3).unwrap().as_str().parse().unwrap();

        for val in lower_bound..upper_bound + 1 {
            let val_s = val.to_string();
            if val_s.len() % 2 != 0 {
                continue;
            }
            let first_half: i128 = val_s[0..val_s.len() / 2].chars().collect::<String>().parse().unwrap();
            let second_half: i128 = val_s[val_s.len() / 2..val_s.len()].chars().collect::<String>().parse().unwrap();

            if first_half == second_half {
                if verbose{println!("{} {} {} | ({} -- {} -- {}) | {}", first_half, "==".to_string().green(), second_half,lower_bound.to_string().cyan(), val_s.to_string().bright_yellow().bold(),upper_bound.to_string().cyan(),score.to_string().bright_magenta());}
                score = score + val;
            }
        }
    }
    return score;
}
fn d_2_part_2(verbose:bool) -> i128 {

    let mut score:i128=0;
    let samples = d_2_data();

    for sample in samples {
        let re = Regex::new(r"^(.*?)(-)(.*)$").unwrap();
        let lower_bound: i128 = re.captures(&sample).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let upper_bound: i128= re.captures(&sample).unwrap().get(3).unwrap().as_str().parse().unwrap();

        'p2l2:for val in lower_bound..upper_bound + 1 {
            let val_s = val.to_string();
            for segment_size in 1..val_s.len()/2+1 {
                let segment:Vec<String> = val_s.chars()
                    .collect::<Vec<_>>()
                    .chunks(segment_size)
                    .map(|c| c.iter().collect::<String>())
                    .collect();
                let all_same = segment.windows(2).all(|w| w[0] == w[1]);
                if all_same {
                    score = score + val;
                    if verbose{print!("{:?}{} -- {}",segment,val,score);}
                    continue 'p2l2;
                }
            }
        }
    }
    return score;
}
pub fn d_2_data() -> Vec<&'static str>{
     let ranges: Vec<&str> = vec![
        "24-46",
        "124420-259708",
        "584447-720297",
        "51051-105889",
        "6868562486-6868811237",
        "55-116",
        "895924-1049139",
        "307156-347325",
        "372342678-372437056",
        "1791-5048",
        "3172595555-3172666604",
        "866800081-866923262",
        "5446793-5524858",
        "6077-10442",
        "419-818",
        "57540345-57638189",
        "2143479-2274980",
        "683602048-683810921",
        "966-1697",
        "56537997-56591017",
        "1084127-1135835",
        "1-14",
        "2318887654-2318959425",
        "1919154462-1919225485",
        "351261-558210",
        "769193-807148",
        "4355566991-4355749498",
        "809094-894510",
        "11116-39985",
        "9898980197-9898998927",
        "99828221-99856128",
        "9706624-9874989",
        "119-335",
    ];
    return ranges;//[1..10].to_vec();
}
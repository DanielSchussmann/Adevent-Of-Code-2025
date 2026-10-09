
pub fn day_3(verbose:bool) -> String{
    let p1 = d3_p1(verbose);
    let p2 = d3_p2(verbose);
    let output:String = format!("Part 1:{}\nPart 2:{}",p1.to_string().yellow().bold(),p2.to_string().yellow().bold());


output}

fn d3_p1(verbose:bool) -> i128{
    let input =  d_3_data();
    let mut running_output:i128 = 0;

    for run in input {
        let chucky: Vec<String> = run.chars().collect::<Vec<_>>()
            .chunks(100)
            .map(|c| c.iter().collect::<String>())
            .collect();
        let mut output: Vec<i128> = vec![0, 0, 0]; //0 = Potential, 1 = i, 2 = j

        for chunk in chucky {
            let cur_chunk = chunk.chars().collect::<Vec<_>>();
            let cur_chunk_nums: Vec<u32> = cur_chunk.iter()
                .filter_map(|s| s.to_digit(10))   // no unwrap! Option goes in, filter_map handles it
                .collect();
            for i in 0..cur_chunk_nums.len() {
                for j in i + 1..cur_chunk_nums.len() {
                    let potential: i128 = (cur_chunk_nums[i].to_string() + &cur_chunk_nums[j].to_string()).parse().unwrap();

                    if potential > output[0] {
                        let pos_1: i128 = cur_chunk_nums[i] as i128;
                        let pos_2: i128 = cur_chunk_nums[j] as i128;
                        output = vec![potential, pos_1, pos_2];
                        if verbose{println!("({}{};{}{}) {:?} ", i,cur_chunk_nums[i],j, cur_chunk_nums[j], output);}
                    }
                }
            }
        }

        running_output = running_output + &output[0];
    }
    
    running_output
}




fn d3_p2(verbose:bool) -> i128{
    let input =  d_3_data();
    let mut running_output:i128 = 0;

    for run in input {
        let show_text:String = run.to_string();
        let chucky: Vec<String> = run.chars().collect::<Vec<_>>()
            .chunks(100)
            .map(|c| c.iter().collect::<String>())
            .collect();


        let mut output: Vec<i128> = vec![0,0,0,0,0,0,0,0,0,0,0,0]; //0 = Potential, 1 = i, 2 =
        let mut visualize:Vec<usize> = vec![];
        let mut number_output:i128 = 0;

        for chunk in chucky {
            let cur_chunk = chunk.chars().collect::<Vec<_>>();
            let cur_chunk_nums: Vec<u32> = cur_chunk.iter()
                .filter_map(|s| s.to_digit(10))   // no unwrap! Option goes in, filter_map handles it
                .collect();

            let mut pos:usize= 11;
            let mut j =0;
            let mut pos_of_i:usize =0;

            loop{
                for i in j..cur_chunk_nums.len() - pos {
                    let potential: i128 = cur_chunk_nums[i] as i128;

                    if potential > output[pos] {
                        output[pos] = potential;
                        pos_of_i = i ;
                }

                }
                number_output =number_output + output[pos] * 10_u128.pow(pos as u32) as i128;
                visualize.push(pos_of_i);

                if pos == 0{
                    break
                }
                pos-=1;
                j = pos_of_i+1;

            }
        }


        if verbose {
            //println!("{}", show_text);
            print!(" {}{}", &show_text[0..visualize[0]], &show_text[visualize[0]..visualize[0] + 1].bold().bright_cyan());
            for i in 1..visualize.len() {
                let cur = visualize[i];
                let prev = visualize[i - 1];
                print!("{}{}", &show_text[prev + 1..cur], &show_text[cur..cur + 1].bold().bright_cyan());

            }
            print!("\n ↪{}\n\n",number_output.to_string().bold().bright_green());
        }



        running_output += number_output
    }

     running_output
}


pub fn d_3_data() -> Vec<&'static str> {
    let contents: &'static str = include_str!("inputs/d3_input.txt");
    contents.lines().collect()
}

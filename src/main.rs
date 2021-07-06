use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "lottery")]
struct Opt {
    /// Options or option count
    #[structopt(short, long)]
    options: Vec<String>,

    /// Set prob manually (default is uniform)
    #[structopt(short, long)]
    prob: Vec<f64>,

    /// How many items to sample
    #[structopt(short, long, default_value = "1")]
    num: usize,

    // TODO add replacement here
}


fn main() -> Result<(), &'static str> {
    let opt: Opt = Opt::from_args();
    let use_number_items = opt.options.len() == 1;
    let selection_count = if use_number_items {
        opt.options[0].parse::<usize>().unwrap()
    } else {
        opt.options.len() as usize
    };
    if selection_count <= 1 {
        return Err("Proved options or valid option count");
    }
    let weight = if opt.prob.len() <= 0 {
        vec![1.0; selection_count]
    } else if opt.prob.len() == selection_count {
        opt.prob.clone()
    } else {
        return Err("Prob list should have same length as option count");
    };
    let options = if use_number_items {
        (0..selection_count).map(|x| format!("Item{}", x + 1)).collect()
    } else {
        opt.options.clone()
    };

    let dist = WeightedIndex::new(&weight).unwrap();
    let mut rng = thread_rng();

    let mut count_of_result = vec![0 as usize; selection_count];
    for i in 0..opt.num {
        let sampled_index = dist.sample(&mut rng);
        println!("Round {}, result is {}", i + 1, options.get(sampled_index).unwrap());
        count_of_result[sampled_index] += 1;
    }

    if opt.num > 1 {
        println!("Summary of lottery:");
        for i in 0..selection_count {
            if count_of_result[i] > 0 {
                println!("Item {} was selected {} times", options[i], count_of_result[i])
            }
        }
    }

    Ok(())
}
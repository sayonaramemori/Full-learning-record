use std::env;
use std::io::{Read,Write,self,BufRead,BufReader};
use std::io::prelude::*;

//get a buffer reader, from a markdown file
fn read(file_name:&str) -> BufReader<std::fs::File>
    {
        let reader = std::fs::File::open(file_name).expect("file open failed");
        let buf = BufReader::new(reader);
        buf
    }

//get the max <h> level
fn get_max_hie(headlines:&Vec<(String,u32)>) -> usize 
    {
            let mut max_hie:usize = 6;
            let _ = headlines
                .into_iter()
                .filter_map(|(l,_)|{l.find(' ')}) //filter_map is for Option
                .map(|val|{if val<max_hie {max_hie = val;}})
                .last();
            max_hie
    }

//extract the headlines from the buffer
fn get_headlines(buf_reader:BufReader<std::fs::File>) -> Vec<(String,u32)>
    {
        let res = buf_reader.lines()
            .filter_map(|l| l.ok())
            .zip(0..)
            .filter(|(l,_)| l.starts_with("#")||l.starts_with("```"))
            .collect::<Vec<(String,u32)>>();
        let mut flag = true;
        let headlines = res
            .into_iter()
            .filter(move |(l,_)| {if l.starts_with("```"){flag=!flag;return false;};flag})
            .map(|(l,i)| (l.clone(),i))
            .collect::<Vec<(String,u32)>>();
        headlines
    }

#[derive(Clone,Debug,Default)]
struct Item_content
{
    line:usize,
    relative_hie:usize,
    prefix:String,
    description:String,
    anchor:String,
}

fn gen_content(headlines:&Vec<(String,u32)>) -> Vec<String> 
    {
        let max_hie = get_max_hie(&headlines);
        let mut res = vec![Item_content::default();headlines.len()];
        let hie = headlines.into_iter()
            .filter_map(|(l,_)|{l.find(' ')})
            .map(|val| val)
            .collect::<Vec<usize>>();
        //let description = headlines.into_iter()
//        let mut hie:[usize;7] = [0;7];
//        println!("{:?}",hie);
        let res = hie.into_iter()
            .zip(headlines)
            .map(|(n,hl)|{
                let mut res=String::from("\t").repeat(n-max_hie);
                res=res+"["+&hl.0[n+1..].trim()+"](#"+")";
                res
            })
            .inspect(|l| println!("{:?}",l))
            .collect::<Vec<String>>();
        res
    }

fn main()
    {
        //ignore the program name
        let mut args = env::args().into_iter().skip(1);
        if let Some(file) = args.next() {
            let buf_reader = read(&file);
            //buf_reader lose its ownship
            let headlines = get_headlines(buf_reader);
            let content = gen_content(&headlines);
        }else{
            println!("Please offer a markdown file");
        }
    }

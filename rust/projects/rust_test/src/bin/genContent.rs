use std::env;
use std::io::{Read,Write,self,BufRead,BufReader,BufWriter};
fn read_from(file_name:&str) -> BufReader<std::fs::File>
    {
        let reader = std::fs::File::open(file_name).expect("file open failed");
        let buf = BufReader::new(reader);
        buf
    }
fn get_max_hie(headlines:&Vec<(String,usize)>) -> usize 
    {
            let mut max_hie:usize = 6;
            let _ = headlines.into_iter()
                .filter_map(|(l,_)|{l.find(' ')}) //filter_map is for Option
                .map(|val|{if val<max_hie {max_hie = val;}})
                .last();
            max_hie
    }
fn get_headlines(buf_reader:BufReader<std::fs::File>) -> Vec<(String,usize)>
    {
        let res = buf_reader.lines()
            .filter_map(|l| l.ok())
            .zip(0..)
            .filter(|(l,_)| l.starts_with("#")||l.starts_with("```"))
            .collect::<Vec<(String,usize)>>();
        let mut flag = true;
        let headlines = res.into_iter()
            .filter(move |(l,_)| {if l.starts_with("```"){flag=!flag;return false;};flag})
            .collect::<Vec<(String,usize)>>();
        headlines
    }
fn gen_content(headlines:&Vec<(String,usize)>) -> Vec<String> 
    {
        let max_hie = get_max_hie(&headlines);
        let hie = headlines.into_iter()
            .filter_map(|(l,_)|{l.find(' ')})
            .map(|val| val-max_hie)
            .collect::<Vec<usize>>();
        let mut temp_hie = [0usize;6];
        let num = (&hie).into_iter()
            .map(move |val| {
                let val = *val;
                temp_hie[val]+=1;
                for i in val+1..temp_hie.len(){temp_hie[i]=0;}
                let mut res = String::new();
                for i in &mut temp_hie{
                    if *i == 0 { break; }
                    res = res + &i.to_string() + ".";
                }
                res.remove(res.len()-1);
                res})
            .collect::<Vec<_>>();
        let mut res = vec![];
        for i in 0..headlines.len() {
            let mut content=String::from("\t").repeat(hie[i]);
            content=content+"- ["+&num[i]+" "+headlines[i].0[hie[i]+1+max_hie..].trim()+"](#"+&headlines[i].1.to_string()+")";
            res.push(content);
        }
        res
    }
fn write_to(dest:&str,content:Vec<String>,mut headlines:Vec<(String,usize)>) 
    {
        let buf_reader = read_from(dest);
        let dest = "CopyVersion_".to_string() + dest; 
        let writer = std::fs::File::create(dest).unwrap();
        let mut buf_writer = BufWriter::new(writer);
        for line in content {writeln!(buf_writer,"{}",line);}
        let mut index:usize = 0;
        let mut index_hl:usize = 0;
        headlines.push((String::new(),0usize));
        let _ = buf_reader.lines()
            .filter_map(|l| l.ok())
            .map(|l| {
                if index == headlines[index_hl].1 {
                    writeln!(buf_writer,"{}<a id='{}'></a>  ",headlines[index_hl].0.trim(),headlines[index_hl].1);
                    index_hl += 1;
                }else {
                    writeln!(buf_writer,"{}",l);
                }
                index+=1;
            })
            .last();
        buf_writer.flush();
    }
fn main()
    {
        let mut args = env::args().into_iter().skip(1);
        if let Some(file) = args.next() {
            let buf_reader = read_from(&file);
            let headlines = get_headlines(buf_reader);
            let content = gen_content(&headlines);
            write_to(&file,content,headlines);
        }else{println!("Please offer a markdown file");}
    }

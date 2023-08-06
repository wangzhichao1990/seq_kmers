use bio::io::fasta;
use bio::io::fasta::Record;
use std::io::Write;
use std::{env, fs};

fn main() {
    let mut args = env::args();
    args.next();
    let fasta_file = match args.next() {
        None => {
            panic!("请指定fasta文件");
        }
        Some(f) => f,
    };
    let size = match args.next() {
        None => {
            panic!("请指定kmer大小");
        }
        Some(size) => size.parse::<usize>().unwrap(),
    };
    let out = match args.next() {
        None => {
            panic!("请指定结果文件")
        }
        Some(out) => out,
    };
    let reader = fasta::Reader::from_file(fasta_file).expect("读取文件失败");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(out)
        .expect("创建文件失败");
    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        let vec = build_kmers(&record, size);
        let seq_id = record.id();
        for (i, x) in vec.iter().enumerate() {
            file.write_all(format!(">{}.{}\n", seq_id, i).as_ref())
                .expect("写入失败");
            file.write_all(x).expect("写入失败");
            file.write(b"\n").expect("写入失败");
        }
    }
}

fn build_kmers(record: &Record, size: usize) -> Vec<&[u8]> {
    let mut kmers = vec![];
    let n_kmers = record.seq().len() - size + 1;
    for i in 1..n_kmers {
        let kmer = &record.seq()[i..(i + size)];
        kmers.push(kmer);
    }
    return kmers;
}

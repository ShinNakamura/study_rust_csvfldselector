use std::io::{self};

type MyResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> MyResult {
    let flds = get_flds();
    let input = io::stdin();
    let input = io::BufReader::new(input.lock());
    let mut rdr = csv::Reader::from_reader(input);
    let stdout = io::stdout();
    let out = io::BufWriter::new(stdout.lock());
    let mut wtr = csv::Writer::from_writer(out);

    // 抜き取り対象のフィールドが入力CSVの中で何番目の列なのかを検査
    // また、確実に入力CSVにも存在するフィールド名のみ収集する
    // つまり、コマンドライン引数には指定されたが実際にはないフィールド名は無視する
    let mut header: Vec<String> = Vec::new();
    let mut fld_nums: Vec<usize> = Vec::new();
    for fld in flds.iter() {
        for (i, column) in rdr.headers()?.iter().enumerate() {
            if fld == column {
                fld_nums.push(i);
                header.push(column.to_string());
            }
        }
    }
    wtr.write_record(&header)?;
    for result in rdr.records() {
        let record_all = result?;
        let mut record: Vec<String> = Vec::new();
        // コマンドライン引数で指定した順番でpushする
        for i in fld_nums.iter() {
            record.push(record_all[*i].to_string());
        }
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn get_flds() -> Vec<String> {
    let mut flds: Vec<String> = Vec::new();
    let args = std::env::args();
    if args.len() < 2 {
        return flds;
    }
    let mut is_cmd_name = true;
    for arg in args {
        if is_cmd_name {
            is_cmd_name = false;
            continue;
        }
        flds.push(arg.to_string());
    }
    flds
}

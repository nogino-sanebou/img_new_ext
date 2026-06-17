use std::fs;
use anyhow::Error;

fn main() {

}

fn load(path: &str) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(path)?)
}

fn write(path: &str, target: Vec<u8>) -> anyhow::Result<()> {
    Ok(fs::write(path, target)?)
}

fn encode_image(input: &str, output: &str) -> anyhow::Result<()> {
    let data = if let Ok(data) = load(input) {
        data
    } else {
        return Err(Error::msg("読み込み処理に失敗しました。"));
    };

    let data = encode(data);
    let data = match data {
        Ok(o) => {
            o
        },
        Err(e) => {
            return Err(e);
        },
    };

    if let Err(e) = write(output, data) {
        return Err(e);
    }

    Ok(())
}

fn encode(target: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let mut out = Vec::new();

    let mut target = &target[..];
    let mut t = 0;
    if let Some((fst, data)) = target.split_first() {
        t = *fst;
        target = data;
    }

    let mut count = 1;
    while let Some((fst, data)) = target.split_first() {
        if data.len() == 0 {
            if t == *fst {
                count += 1;
            }

            out.push(t);
            out.push(count);
            break;
        }

        if t == *fst {
            count += 1;

            if count == 255 {
                out.push(t);
                out.push(255);
                count = 0;
            }

            target = data;
            continue;
        }

        if count > 1 {
            out.push(t);
            out.push(count);

            t = *fst;
            count = 1;

            target = data;
            continue;
        }

        out.push(t);
        out.push(1);
        t = *fst;
        target = data;
    }

    Ok(out)
}

fn decode_image(input: &str, output: &str) -> anyhow::Result<()> {
    let data = if let Ok(data) = load(input) {
        data
    } else {
        return Err(Error::msg("読み込み処理に失敗しました。"));
    };

    let data = decode(data);
    let data = match data {
        Ok(o) => {
            o
        },
        Err(e) => {
            return Err(e);
        },
    };

    if let Err(e) = write(output, data) {
        return Err(e);
    }

    Ok(())
}

fn decode(target: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    if target.len() < 2 {
        return Err(Error::msg("入力ファイルのデータ構成が不正です。"));
    }

    let mut pos = 0;
    let mut out = Vec::new();

    loop {
        if pos >= target.len() - 1 {
            break;
        }

        let data = vec![target[pos]; target[pos + 1] as usize];
        out.extend(data);

        pos += 2;
    }

    Ok(out)
}

#[cfg(test)]
mod test {
    use crate::{decode_image, encode_image};

    #[test]
    fn test1() {
        encode_image("img.bmp", "img.sif").unwrap();
        decode_image("img.sif", "img2.bmp").unwrap();
    }

}
use criterion::{criterion_group, criterion_main, Criterion};
use sigma::Sigma;
const INPUT_1KB: &str = include_str!("./small_data_1kb.txt");
const INPUT_10KB: &str = include_str!("./small_data_10kb.txt");
const INPUT_50KB: &str = include_str!("./small_data_50kb.txt");
const INPUT_500KB: &str = include_str!("./small_data_500kb.txt");
const INPUT_1MB: &str = include_str!("./small_data_1mb.txt");

fn small_data_parse(c: &mut Criterion) {
  let sigma_small_1kb = Sigma::new(INPUT_1KB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  c.bench_function("small_data_1kb_parse", move |b| {
    b.iter_with_setup(
      || sigma_small_1kb.clone(),
      |sigma| sigma.parse().unwrap(),
    );
  });

  let sigma_small_10kb = Sigma::new(INPUT_10KB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  c.bench_function("small_data_10kb_parse", move |b| {
    b.iter_with_setup(
      || sigma_small_10kb.clone(),
      |sigma| sigma.parse().unwrap(),
    );
  });

  let sigma_small_50kb = Sigma::new(INPUT_50KB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  c.bench_function("small_data_50kb_parse", move |b| {
    b.iter_with_setup(
      || sigma_small_50kb.clone(),
      |sigma| sigma.parse().unwrap(),
    );
  });

  let sigma_small_500kb = Sigma::new(INPUT_500KB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  c.bench_function("small_data_500kb_parse", move |b| {
    b.iter_with_setup(
      || sigma_small_500kb.clone(),
      |sigma| sigma.parse().unwrap(),
    );
  });
  let sigma_small_1mb = Sigma::new(INPUT_1MB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  c.bench_function("small_data_1mb_parse", move |b| {
    b.iter_with_setup(
      || sigma_small_1mb.clone(),
      |sigma| sigma.parse().unwrap(),
    );
  });
}

fn small_data_compile(c: &mut Criterion) {
  let sigma_small_1kb = Sigma::new(INPUT_1KB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  let sigma_small_1kb = sigma_small_1kb.parse().unwrap();
  c.bench_function("small_data_1kb_compile", move |b| {
    b.iter_with_setup(
      || sigma_small_1kb.clone(),
      |sigma| sigma.compile().unwrap(),
    );
  });

  let sigma_small_10kb = Sigma::new(INPUT_1KB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  let sigma_small_10kb = sigma_small_10kb.parse().unwrap();
  c.bench_function("small_data_10kb_compile", move |b| {
    b.iter_with_setup(
      || sigma_small_10kb.clone(),
      |sigma| sigma.compile().unwrap(),
    );
  });

  let sigma_small_50kb = Sigma::new(INPUT_50KB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  let sigma_small_50kb = sigma_small_50kb.parse().unwrap();
  c.bench_function("small_data_50kb_compile", move |b| {
    b.iter_with_setup(
      || sigma_small_50kb.clone(),
      |sigma| sigma.compile().unwrap(),
    );
  });
  let sigma_small_500kb = Sigma::new(INPUT_500KB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  let sigma_small_500kb = sigma_small_500kb.parse().unwrap();
  c.bench_function("small_data_500kb_compile", move |b| {
    b.iter_with_setup(
      || sigma_small_500kb.clone(),
      |sigma| sigma.compile().unwrap(),
    );
  });
  let sigma_small_1mb = Sigma::new(INPUT_1MB)
    .bind("sigma_one", "sigma1")
    .bind("sigma_two", "sigma2");
  let sigma_small_1mb = sigma_small_1mb.parse().unwrap();

  c.bench_function("small_data_1mb_compile", move |b| {
    b.iter_with_setup(
      || sigma_small_1mb.clone(),
      |sigma| sigma.compile().unwrap(),
    );
  });
}
criterion_group!(benches, small_data_parse, small_data_compile);
criterion_main!(benches);

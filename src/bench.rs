use super::test::Bencher;
use super::translator::*;

#[bench]
fn bench_example_13(b: &mut Bencher) {
    let input = "***\n---\n___";
    b.iter(|| exec(input))
}

#[bench]
fn bench_example_32(b: &mut Bencher) {
    let input = "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo";
    b.iter(|| exec(input))
}

#[bench]
fn bench_example_50(b: &mut Bencher) {
    let input = "Foo *bar*\n=========\n\nFoo *bar*\n---------";
    b.iter(|| exec(input))
}

#[bench]
fn bench_example_76(b: &mut Bencher) {
    let input = "    a simple\n      indented code block";
    b.iter(|| exec(input))
}
#[bench]
fn bench_example_91(b: &mut Bencher) {
    let input = "```\naaa\n~~~\n```";
    b.iter(|| exec(input))
}

#[bench]
fn bench_example_160(b: &mut Bencher) {
    let input = "   [foo]: \n      /url  \n           'the title'  \n\n[foo]";
    b.iter(|| exec(input))
}

#[bench]
fn bench_example_182(b: &mut Bencher) {
    let input = "aaa\n\nbbb";
    b.iter(|| exec(input))
}

#[bench]
fn bench_example_190(b: &mut Bencher) {
    let input = "  \n\naaa\n  \n\n# aaa\n\n  ";
    b.iter(|| exec(input))
}

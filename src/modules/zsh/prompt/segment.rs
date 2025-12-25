pub fn segment(command: Box<zsh_prompts::Commands>) {
    let segs = command.exec();
    let segs: String = segs
        .into_iter()
        .map(|seg| seg.format())
        .collect::<Vec<String>>()
        .join(" ");
    print!("{}", segs);
}

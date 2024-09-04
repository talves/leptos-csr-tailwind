pub mod converter;

struct Collection<'a> {
    name: &'a str,
}
struct File<'a> {
    name: &'a str,
}
struct Folder<'a> {
    name: &'a str,
}

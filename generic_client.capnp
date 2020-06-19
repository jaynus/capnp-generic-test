@0xda82a5acd0c201f5;

interface Worker (S) {
    getInterface @0 () -> (interface: S);
}

interface A {
    test @0 (request: Text) -> (reply: Text);
}

interface B {
    test @0 (request: Text) -> (reply: Text);
}
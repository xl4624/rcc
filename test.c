int return5again() {
    return 5;
}

int return5() {
    return return5again();
}

int main() {
    return return5();
}

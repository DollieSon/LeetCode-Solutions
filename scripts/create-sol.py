def main():
    file_name = input("Enter question number: ")
    file_loc = "../rust-leet/src/Solutions/"
    with open(file_loc + "leet" + file_name + ".rs", "w") as f:
        f.write("use macros::sol_macro;\n")
        f.write("sol_macro!();\n")
    
    with open(file_loc + "mod.rs", "a") as f:
        f.write("\npub mod leet" + file_name + ";")


if __name__ == "__main__":
    main()
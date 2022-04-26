#!python

import json


def main():

    with open("./pkg-single/package.json", "r") as f:
        j = json.load(f)
        j['name'] = j['name'] + "_single"

    with open("./pkg-single/package.json", "w") as f:
        json.dump(j, f, ensure_ascii=False, indent=4)


if __name__ == "__main__":
    main()

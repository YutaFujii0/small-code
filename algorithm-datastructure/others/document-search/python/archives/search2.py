import sys
import re

FILEPATH = '../../articles-202109/HelloWorld.txt'

def main():
    if len(sys.argv) < 2:
        return display_error()
    print('Searching for keyword in HelloWorld.txt')
    keyword = sys.argv[1]
    filepath = FILEPATH
    found = search(keyword, filepath)
    print(f'Result: {found}')

def search(keyword, filepath):
    found = False
    with open(filepath) as f:
        for word in re.split('\W', f.read()):
            if keyword.lower() == word.lower():
                found = True
                break
    return found

def display_error():
    print('Search failed. Please give me some keyword.')
    print('Expample:')
    print('  python3 search2.py startup')

if __name__ == '__main__':
    main()

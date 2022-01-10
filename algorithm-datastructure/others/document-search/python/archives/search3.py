import sys
import re
import os

ARTICLES_PATH = './articles-202109/'

def main():
    if len(sys.argv) < 2:
        return display_error()
    keyword = sys.argv[1]
    print(f'Searching for "{keyword}"')
    hits = []
    for filename in os.listdir(ARTICLES_PATH):
        found = search(keyword, os.path.join(ARTICLES_PATH, filename))
        if found: hits.append(filename)
    print('Result:')
    for index, filename in enumerate(hits):
        title = re.sub("\.txt$", "", filename)
        print(f'{index + 1}. {title}')
    print(f'\nTotal: {len(hits)}')

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


"""
とりあえず動くものをつくった
"""
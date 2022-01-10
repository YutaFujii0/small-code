import sys
import re
import os

def main():
    try:
        if len(sys.argv) < 2:
            raise NoArgumentError
        keyword = sys.argv[1]
        if re.search('\w', keyword) is None:
            raise InvalidArgumentError
        ArticleSearchService(keyword).call()
    except Error as err:
        print(err.message)

class ArticleSearchService:
    ARTICLES_PATH = '../articles-202109/'

    def __init__(self, keyword):
        self.keyword = keyword
        self.hits = []

    def call(self):
        self.__display_start_msg()
        self.__search()
        self.__display_result()

    def __display_start_msg(self):
        print(f'Searching for "{self.keyword}"')

    def __display_result(self):
        print('Result:')
        for index, article in enumerate(self.hits):
            print(f'{index + 1}. {article.title}')
        print(f'\nTotal: {len(self.hits)}')

    def __search(self):
        for filename in os.listdir(self.ARTICLES_PATH):
            article = Article.find(os.path.join(self.ARTICLES_PATH, filename))
            if article is not None and article.contains(self.keyword):
                self.hits.append(article)

class Article:
    def __init__(self, filepath):
        self.filepath = filepath
        self.title = re.sub("\.txt$", "", filepath.split('/').pop())

    def contains(self, keyword):
        found = False
        with open(self.filepath) as f:
            for word in re.split('\W', f.read()):
                if re.search('\w', word) is None:
                    continue
                if keyword.lower() == word.lower():
                    found = True
                    break
        return found

    @classmethod
    def find(cls, filepath):
        if os.path.exists(filepath):
            return cls(filepath)
        else:
            return None

class Error(Exception):
    message = 'An error occured.'

class NoArgumentError(Error):
    message = 'Search failed. Please give me some keyword.\nExpample:\n  python3 search2.py startup'

class InvalidArgumentError(Error):
    message = 'Search failed. Keyword must be a word(a-z, 0-9).\nExpample:\n  python3 search2.py word2vec'

if __name__ == '__main__':
    main()


"""
Pythonでの着眼点

- pythonには言語としてのprivateメソッドはない．しかし慣例として'__'を先頭につけることが多い
- PythonにはSafety Operatorがないため，冗長な書き方になる（Article.findのあと）


オブジェクト指向を導入
（オブジェクト指向の性質として，Encapslation，Inheritance，．．がある）

ArticleはMVCにおけるModel的役割
クラスメソッドとして，検索という関数を持たせる
インスタンスメソッドとして，特定キーワードを含むかどうかの関数

検索サービスクラス
instantiateとcallメソッドだけを持つ．他はプライベートメソッド
検索する，結果を返すという役割のみを持つので，検索する必要がない場合のエラーメッセージなどはこのクラスに責任を持たせない


search3-2.py
主にエラーハンドリングを追加

- 引数が '"のとき全部ヒットしてしまう
"""

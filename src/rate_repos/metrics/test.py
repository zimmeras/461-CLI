import requests
import config
import os

def calculate(repo_url:str):
    val = star_score(repo_url)
    path = './repos/'
    testpath = './repos/test/'
    num = traverse_files_for_asserts(testpath)
    lines = traverse_files_for_sloc(path)
    assert_ratio = num / lines
  #  print("star score: " + str(val))
  #  print("test score: " + str(assert_ratio))
    fittedval = 1 - (1.1)**(-0.0012 * val)
    adjusted_ratio = 1 - (2.2)**(-32 * assert_ratio)
  #  print(fittedval)
  #  print(adjusted_ratio)

    final_score = 0.3 * fittedval + 0.7 * adjusted_ratio
  #  print(final_score)
    return final_score


def star_score(repo_url):
    gh_session = requests.Session()
    gh_session.auth = (config.GIT_USERNAME, config.GIT_TOKEN)
    repo_info = repo_url[19:].split('/')
    owner = repo_info[0]
    repo = repo_info[1]
    github_api = "https://api.github.com/graphql"

    query = "query{repository(owner:\""+ owner + "\", name:\"" + repo + "\"){stargazerCount}}"
    response = gh_session.post(url=github_api,json={"query":query})
    info = response.json()
    stars = info['data']['repository']['stargazerCount']
    return(stars)

def traverse_files_for_sloc(path):
    total = 0
    with os.scandir(path) as parent:
        for item in parent:
            if item.is_file():
                file_type = item.name.split('.')
                for i in file_type:
                    if i in 'jsxtsxpygo':
                        total += count_lines(item)
            elif item.is_dir():
                total += traverse_files_for_sloc(item.path)
    return total

def count_lines(file):
    with open(file) as fp:
        lines = fp.readlines()
    total = len(lines)
    for i in range(len(lines)):
        if '/*' in lines[i]:
            while('*/' not in lines[i] and i + 1 < len(lines)):
                total -= 1
                i += 1
        if len(lines[i]) == 1:
            total -= 1
        if '//' in lines[i]:
            total -= 1
        
    return(total)

def traverse_files_for_asserts(path):
    total = 0
    with os.scandir(path) as parent:
        for item in parent:
            if item.is_file():
                file_type = item.name.split('.')
                for i in file_type:
                    if i in 'jsxtsxpygo':
                        total += scan_file_for_asserts(item)
            elif item.is_dir():
                total += traverse_files_for_asserts(item.path)
    return total

def scan_file_for_asserts(file):
    asserts = 0
    with open(file) as f:
        for line in f:
            if 'assert.' in line:
                asserts += 1
            if '\'assert.\''in line:
                asserts -= 1
            if '\"assert.\"' in line:
                asserts -= 1

    if asserts < 0: asserts = 0
    return(asserts)
    

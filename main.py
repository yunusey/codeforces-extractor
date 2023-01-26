import requests, sys, os


def write_to_file(path: str, text: str) -> None:
    file = open(f'{path}', 'w')
    file.write(text)
    file.close()

def writeProblems(path: str, problems: dict) -> None:
    path = path + '/' if not path.endswith('/') else path
    for i in problems.keys():
        print(f"Problem #{i}:", problems[i])
        folder_name = path + str(i) + '/'
        if not os.path.isdir(folder_name):
            os.makedirs(folder_name)
        for tc_ind, tc in enumerate(problems[i], 1):
            file_name_input  = folder_name + "I" + str(tc_ind)
            file_name_output = folder_name + "O" + str(tc_ind)
            write_to_file(file_name_input,  tc[0])
            write_to_file(file_name_output, tc[1])


def getInputPrompts(text: str) -> list:
    ans = []
    pattern = '<div class="input"><div class="title">Input</div><pre>'
    end = '</pre></div>'
    while True:
        ind = text.find(pattern)
        if ind == -1:
            break
        text = text[ind + len(pattern) : ]
        end_ind = text.find(end)
        s = text[ : end_ind]
        ans.append(s)

    return ans

def getOutputPrompts(text: str) -> list:
    ans = []
    pattern = '<div class="output"><div class="title">Output</div><pre>'
    end = '</pre></div>'
    while True:
        ind = text.find(pattern)
        if ind == -1:
            break
        text = text[ind + len(pattern) : ]
        end_ind = text.find(end)
        s = text[ : end_ind]
        ans.append(s)

    return ans

def parseLongText(text: str):
    pattern = '<div class="test-example-line'
    index = text.find(pattern)
    next_endl = index
    while next_endl < len(text) and text[next_endl] != '\n':
        next_endl += 1
    line = text[index : next_endl]
    list_div = line.split('</div>')
    last = ""

    for i in list_div:
        if not i.startswith(pattern):
            break
        s = ""
        for j in range(len(i) - 1, -1, -1):
            if i[j] == '>':
                break
            s = i[j] + s
        last += s + '\n'

    last = last[:-1]
    return last


def parseInput(a: list) -> list:
    ans = []
    for i in a:
        pattern = '<div class="test-example-line'
        if pattern in i:
            i = parseLongText(i)
            if i.startswith('\n'):
                i = i[1:]
            if i.endswith('\n'):
                i = i[:-1]
            ans.append(i)
        else:
            i = str(i)
            if i.startswith('\n'):
                i = i[1:]
            if i.endswith('\n'):
                i = i[:-1]
            ans.append(i)
    return ans


def parseOutput(a: list) -> list:
    ans = []
    for i in a:
        i = str(i)
        if i.startswith('\n'):
            i = i[1:]
        if i.endswith('\n'):
            i = i[:-1]
        ans.append(i)
    return ans

def getProblem(link: str, problem: str) -> tuple:
    complete_link = link + 'problem/' + problem
    message = requests.get(complete_link)
    text = message.text

    input_list = getInputPrompts(text)
    output_list = getOutputPrompts(text)
    input_list = parseInput(input_list)
    output_list = parseOutput(output_list)

    return (input_list, output_list)

def getProblemNames(link: str, number: int) -> list:

    message = requests.get(link)
    text = message.text

    problem_names = []

    while True:
        pattern = f'<a href="/contest/{number}/problem/'
        ind = text.find(pattern)
        if ind == -1:
            break

        if text[ind + len(pattern) + 1] == '"':
            name = text[ind + len(pattern)]
        else:
            name = text[ind + len(pattern) : ind + len(pattern) + 2]
        text = text[ind + len(pattern):]

        if not name in problem_names:
            problem_names.append(name)
    
    return problem_names

def solveForEachProblem(problems: list, link: str) -> dict:
    last = {}
    for i in problems:
        print("Problem #" + i)
        last[i] = []
        getProblem(link, i)
        a, b = getProblem(link, i)
        for j in range(len(a)):
            print("Input #", j + 1)
            print(a[j])
            print("Output #", j + 1)
            print(b[j])
            last[i].append((a[j], b[j]))
        print("---")
    return last

def main(number: int, path: str) -> None:
    link = f'https://codeforces.com/contest/{number}/'
    problems = getProblemNames(link, number)
    last = solveForEachProblem(problems, link)
    print(last)
    writeProblems(path=path, problems=last)

if __name__ == '__main__':
    number = int(sys.argv[1])
    main(number, './test/')

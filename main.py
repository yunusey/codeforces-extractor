import requests, sys, os, time


def write_to_file(path: str, text: str) -> None:
    file = open(f'{path}', 'w')
    file.write(text)
    file.close()

def writeProblems(path: str, problems: dict) -> None:
    path = path + '/' if not path.endswith('/') else path
    for i in problems.keys():
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
    real_ind = 0
    while True:
        ind = text.find(pattern)
        if ind == -1:
            break
        text = text[ind + len(pattern) : ]
        end_ind = text.find(end)
        s = text[ : end_ind]
        real_ind += ind + len(pattern)
        ans.append((s, real_ind))

    return ans

def getOutputPrompts(text: str) -> list:
    ans = []
    pattern = '<div class="output"><div class="title">Output</div><pre>'
    end = '</pre></div>'
    real_ind = 0
    while True:
        ind = text.find(pattern)
        if ind == -1:
            break
        text = text[ind + len(pattern) : ]
        end_ind = text.find(end)
        s = text[ : end_ind]
        real_ind += ind + len(pattern)
        ans.append((s, real_ind))

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


def parseInput(i: str) -> str:
    pattern = '<div class="test-example-line'
    if pattern in i:
        i = parseLongText(i)
        if i.startswith('\n'):
            i = i[1:]
        if i.endswith('\n'):
            i = i[:-1]
    else:
        i = str(i)
        if i.startswith('\n'):
            i = i[1:]
        if i.endswith('\n'):
            i = i[:-1]
    return i


def parseOutput(i: str) -> str:
    if i.startswith('\n'):
        i = i[1:]
    if i.endswith('\n'):
        i = i[:-1]
    return i

def getText(link: str) -> str:
    message = requests.get(link)
    text = message.text
    return text

def getProblemNames(text: str) -> list:

    last = []
    pattern = '<div class="header"><div class="title">'
    real_ind = 0
    while True:
        ind = text.find(pattern)
        if ind == -1:
            break
        
        ind += len(pattern)
        problem_name = text[ind] if text[ind + 1] == '.' else text[ind : ind + 2]
        real_ind += ind
        last.append((problem_name, real_ind))
        text = text[ind:]

    return last

def match(problems: list, inputs: list, outputs: list) -> dict:
    last = {}   
    problem_ind = 0
    for index in range(len(inputs)):
        input_prompt, ind = inputs[index]
        output_prompt = outputs[index][0]
        pair = (input_prompt, output_prompt)
        if problem_ind + 1 >= len(problems) or problems[problem_ind + 1][1] > ind:
            if not problems[problem_ind][0] in last:
                last[problems[problem_ind][0]] = []
            last[problems[problem_ind][0]].append(pair)
        else:
            problem_ind += 1
            if not problems[problem_ind][0] in last:
                last[problems[problem_ind][0]] = []
            last[problems[problem_ind][0]].append(pair)

    return last

def main(number: int, path: str) -> None:
    link = f'https://codeforces.com/contest/{number}/problems/'
    text = getText(link)
    problems = getProblemNames(text)
    input_prompts  = getInputPrompts(text)
    output_prompts = getOutputPrompts(text)
    last = match(problems, input_prompts, output_prompts)
    for i in last.keys():
        print("Problem # %s" % i)
        for tc in range(len(last[i])):
            inp, out = last[i][tc]
            inp, out = parseInput(inp), parseOutput(out)
            print("Input: ")
            print(inp)
            print("Output: ")
            print(out)
            last[i][tc] = (inp, out)
    writeProblems(path=path, problems=last)

if __name__ == '__main__':
    number = int(sys.argv[1])
    timer = time.time()
    main(number, './test/')
    diff = time.time() - timer
    print(f"It took {diff} seconds to execute the program...")


import difflib
import re

def file_diff(file1_path, file2_path):
    # 读取文件
    with open(file1_path, 'r') as file1, open(file2_path, 'r') as file2:
        file1_lines = file1.readlines()
        file2_lines = file2.readlines()
    
    # 将文件中的每一行的r"^\++ "替换为""
    file1_lines = [re.sub(r"^\++ ", "", line) for line in file1_lines]
    file2_lines = [re.sub(r"^\++ ", "", line) for line in file2_lines]

    # 保留所有匹配r'^[A-Z]{2,}'的行
    file1_lines = [line for line in file1_lines if re.match(r'^(?:(?:PASS)|(?:SKIP)|(?:XFAIL)|(?:FAIL)|(?:XPASS)|(?:ERROR)):', line)]
    file2_lines = [line for line in file2_lines if re.match(r'^(?:(?:PASS)|(?:SKIP)|(?:XFAIL)|(?:FAIL)|(?:XPASS)|(?:ERROR)):', line)]

    # # 转为两个set，求在2中但不在1中的行
    file1_lines = set(file1_lines)
    file2_lines = set(file2_lines)
    file2_only = file2_lines - file1_lines

    # 将结果转为字符串
    result = ""
    for line in file2_only:
        result += line


    if result != "":
        return result
    else:
        return "No difference found.\n"

if __name__ == "__main__":
    result = "Functional Test Result: \n\n"
    file1_path = '/home/user/test/coreutils-origin/tests/test-suite.log'  # 第一个文件的路径
    file2_path = '/home/user/test/coreutils-rust/tests/test-suite.log'  # 第二个文件的路径
    result += file_diff(file1_path, file2_path)

    print(result)
    

def sum_of_two_digits(a: int ,b: int) -> int:
    return a + b

if __name__ == '__main__':
    print("Please enter the values to add separated by a white space:")
    a, b = map(int, input().split())
    print(f"Sum of {a} and {b} is {sum_of_two_digits(a, b)}")
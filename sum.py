nums = [1,2,3,4]

# [1,3,6,10]

def printsum(arr):
    length = len(arr)
    ans = [0] * length
    ans[0] = arr[0]
    for j in range(1, length):
        print(f"j={j}")
        for i in range(1, j):
            print(f"i={i}")
            print(ans[j])
    return ans


print(printsum(nums))
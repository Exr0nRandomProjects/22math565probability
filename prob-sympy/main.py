from bitarray import test as test_bitarray

from proto import EventPossibilities, Probability, prob


if __name__ == '__main__':
    print("hello world!")
    # pos = EventPossibilities('011')
    # # test_bitarray()
    # print(pos.count())

    p = Probability('unit', [EventPossibilities('001')])
    print(prob("1!", p))

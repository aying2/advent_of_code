import sys

def find_packet_start(filename):
    with open(filename) as ifile:
        line = ifile.readline()
        
        for i, c in enumerate(line):
            
            fourset = [line[i+j] for j in range(0,4)]
            unique = True
            for k in fourset:
                if fourset.count(k) != 1:
                    unique = False

            if(unique):
                return i + 4

def find_message_start(filename):
    with open(filename) as ifile:
        line = ifile.readline()
        
        for i, c in enumerate(line):
            
            fourset = [line[i+j] for j in range(0,14)]
            unique = True
            for k in fourset:
                if fourset.count(k) != 1:
                    unique = False

            if(unique):
                return i + 14
                


filename = sys.argv[1]
print(find_packet_start(filename))
print(find_message_start(filename))


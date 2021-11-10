from sklearn.metrics import f1_score
from sklearn.metrics import recall_score
from sklearn.metrics import precision_score
from sklearn.metrics import accuracy_score

target = []
pred = []
with open("tmp/output") as file:
    for line in file:
        split = line.split(",")
        target.append(int(split[0]))
        pred.append(int(split[1]))

with open("tmp/evaluation", "a") as file:
    file.write("Accuracy: " + str(f1_score(target, pred, average='weighted')) + "\n")
    file.write("F1-Score (Macro): " + str(f1_score(target, pred, average='macro')) + "\n")
    file.write("F1-Score (Micro): " + str(f1_score(target, pred, average='micro')) + "\n")
    file.write("Recall (Macro): " + str(recall_score(target, pred, average='macro')) + "\n")
    file.write("Recall (Macro): " + str(recall_score(target, pred, average='micro')) + "\n")
    file.write("Precision (Micro): " + str(precision_score(target, pred, average='macro')) + "\n")
    file.write("Precision (Micro): " + str(precision_score(target, pred, average='micro')) + "\n")

print("Accuracy: ", f1_score(target, pred, average='weighted'))
print("F1-Score (Macro): ", f1_score(target, pred, average='macro'))
print("F1-Score (Micro): ", f1_score(target, pred, average='micro'))
print("Recall (Macro): ", recall_score(target, pred, average='macro', zero_division=0))
print("Recall (Micro): ", recall_score(target, pred, average='micro', zero_division=0))
print("Precision (Macro): ", precision_score(target, pred, average='macro', zero_division=0))
print("Precision (Micro): ", precision_score(target, pred, average='micro', zero_division=0))
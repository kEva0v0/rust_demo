
fn lastElement(vec: &Vec<usize>) -> usize {
    let mut lastValue = vec[vec.len() - 1];
    return lastValue;
}

fn deal(height: Vec<i32>) -> i32 {
    let mut maxHeight = 0;
    let mut maxHeightIndex = 0;
    let mut length = height.len();
    for index in 0..length {
        maxHeight = if (height[index] > maxHeight) {
            maxHeightIndex = index;
            height[index]
        } else {
            maxHeight
        };
    }

    // construct upgrade queue
    // start from zero
    let mut leftWater = 0;
    let mut upgradeQueue: Vec<usize> = Vec::new();
    let mut answerQueue: Vec<i32> = Vec::new();
    if (maxHeightIndex != 0) {
        upgradeQueue.push(0);
        for i in 1..maxHeightIndex+1 {
            let mut lastEle = lastElement(&upgradeQueue);
            if(height[i] >= height[lastEle]) {
                for j in lastEle..i {
                    answerQueue.push(height[lastEle]);
                } 
                upgradeQueue.push(i);
            }
        }
    }
    answerQueue.push(maxHeight);
    upgradeQueue.clear();
    if (maxHeightIndex != length - 1) {
        upgradeQueue.push(length - 1);
        for i in 1..(length - maxHeightIndex + 1) {
            if(height[length - i] >= height[lastElement(&upgradeQueue)]) {
                upgradeQueue.push(length - i);
            }
        }
        let mut currentIndex = maxHeightIndex;
        for i in 0..upgradeQueue.len() {
            for j in currentIndex..upgradeQueue[upgradeQueue.len() - 1 - i] {
                answerQueue.push(height[upgradeQueue[upgradeQueue.len() - 1 - i]]);
            }
            currentIndex = upgradeQueue[upgradeQueue.len() - 1 - i];
        }
    }
    let mut answer = 0;
    if(answerQueue.len() == length) {
        for i in 0..answerQueue.len() {
            answer = answer + (answerQueue[i] - height[i]);
        }
    }
    return answer;    
}

fn main() {
    let mut height = [0,1,0,2,1,0,1,3,2,1,2,1];
    deal(height.to_vec());
}
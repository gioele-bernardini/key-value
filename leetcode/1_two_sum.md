IDEA => ci sono, dato array di dimensione (n),
sommatoria da 0 a (n-1) combinazioni.



int* twoSum(int* nums, int numsSize, int target, int* returnSize) {
    int* out = (int*) malloc(sizeof(int) * 2);

    int i, j;
    for (i = 0; i < numsSize - 1; i++) {
        for (j = i + 1; j < numsSize; j++) {
            if ((nums[i] + nums[j]) == target) {
                out[0] = i;
                out[1] = j;
                *returnSize = 2;
                return out;
            }
        }
    }

    *returnSize = 0; // Se nessuna soluzione
    return NULL;
}

double findMedianSortedArrays(int* nums1, int nums1Size, int* nums2, int nums2Size)
{
    int m = (nums1Size + nums2Size) / 2;
    int i = 0;
    int j = 0;
    int k = 0;
    double r;
    int *a = malloc((m + 1) * sizeof(int));

    while (i < nums1Size && j < nums2Size && k <= m) {
        if (nums1[i] < nums2[j]) {
            a[k++] = nums1[i++];
        } else {
            a[k++] = nums2[j++];
        }
    }
    while (i < nums1Size && k <= m) {
        a[k++] = nums1[i++];
    }
    while (j < nums2Size && k <= m) {
        a[k++] = nums2[j++];
    }
    r = ((nums1Size + nums2Size) & 1) ? a[m] : (a[m] + a[m - 1]) / 2.0;
    free(a);
    return r;
}

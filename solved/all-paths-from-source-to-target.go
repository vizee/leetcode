func allPathsSourceTarget(graph [][]int) [][]int {
    var res [][]int
    var resolve func (path []int, cur int)
    resolve = func (path []int, cur int) {
        if cur == len(graph) - 1 {
            res = append(res, append([]int(nil), path...))
        } else {
            for _, next := range graph[cur] {
                resolve(append(path, next), next)
            }
        }
    }
    resolve([]int{0}, 0)
    return res
}

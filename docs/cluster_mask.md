### Cluster Mask

```cpp
struct ClusterMask {
    VariableLengthInteger type;  // 31
    VariableLengthInteger mask_length;
    uint8_t* mask;
    size_t byte_size() const {
        return type.byte_size() + mask_length.byte_size() + mask_length();
    }
};
```
<br/>

### Mask structure `Users`
```
user_name@cluster
```
<br/>

Examples
```
akzestia@cx-xa-zw
```
```
zuru@uu-si-xo
```
```
azure@yk-nq-x8
```
<br/>

### Mask structure `Global Services`
```
service@zurui
```
<br/>

Examples
```
authentication@zurui
```
```
cluster_selection@zurui
```
<br/>

### Mask structure `Cluster Specific Services`
```
service@cluster
```
<br/>

Examples
```
status@cx-xa-zw
```
```
media@cx-xa-zw
```
```
archive@cx-xa-zw
```

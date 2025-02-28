# Cross-Cluster Communication Between Users

## **Concept**
Cross-cluster communication enables users to exchange messages and files seamlessly, even when they belong to different clusters. This ensures a smooth user experience without requiring knowledge of the recipient’s cluster details.

---

## **Implementation Details**

### **User Masking**
Each user is identified by a unique masked address that includes their respective cluster:
- **User A** → `user_a@cluster_a`
- **User B** → `user_b@cluster_b`

---

### **Message Flow**
1. **User A sends a message to User B**
   - User A does not know User B’s current cluster location.

2. **Cluster A Processes the Message**
   - The system first checks if User B exists within `cluster_a`.
   - If **User B is found in the same cluster**, the message is delivered immediately.

3. **Routing to the Correct Cluster**
   - If User B is **not** in `cluster_a`, the system retrieves the last known cluster where User B logged in.
   - The message is then redirected to `cluster_b`, the last known cluster of User B.

4. **Cluster B Delivers the Message**
   - `cluster_b` receives the message and delivers it to `user_b@cluster_b`.

---

## **Key Benefits**
- **Seamless Cross-Cluster Communication**: Users can send and receive messages without worrying about cluster-specific details.
- **Efficient Routing**: Ensures messages reach the correct recipient with minimal latency.
- **Scalability**: Supports dynamic user movements across multiple clusters.

---

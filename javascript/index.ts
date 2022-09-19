import { initializeFirestore, doc,addDoc, collection,onSnapshot } from "@firebase/firestore"
import { initializeApp } from "@firebase/app";

const app = initializeApp(
    {
        "apiKey": "AIzaSyByvvP8Rb_uQZnx5cb2BpZj8OvyUxuE2Rc",
        "authDomain": "gagagaga-dev.firebaseapp.com",
        "projectId": "gagagaga-dev",
        "storageBucket": "gagagaga-dev.appspot.com",
        "messagingSenderId": "442174624660",
        "appId": "1:442174624660:web:c860937debeaf770b4b581",
        "measurementId": "G-LS63C7GJ1T"
});

const store = initializeFirestore(app,{})

const NAMESPACE = "rollrole/v1";

const roomCollection = collection(store,NAMESPACE,"rooms");

const syncMembers = (roomId: string,callback: (res: string) => void,onError: () => void): () => void => {
    const memberCol = collection(doc(roomCollection,roomId),"members");
    return onSnapshot(
        memberCol,
        (res) => { callback(JSON.stringify(res.docs.map(doc => doc.data())))},
        onError
    )
}

const addMembers = async (roomId: string): Promise<string> => {
    const memberCol = collection(doc(roomCollection,roomId),"members");
    return (await addDoc(memberCol,{
        id: Math.floor(Math.random() * 10000).toString(),
        name: "test"
    })).id;
}

//@ts-expect-error
window._wasm_js_bridge = {
    addMembers,
    syncMembers
}
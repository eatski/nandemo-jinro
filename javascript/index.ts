import { initializeFirestore, doc,addDoc, collection,onSnapshot, QuerySnapshot, DocumentData, DocumentReference, CollectionReference, Unsubscribe, setDoc } from "@firebase/firestore"
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

const onSnapshotWhenActive = (
    ref: CollectionReference,
    callback: (data: QuerySnapshot<DocumentData>) => void,
    onError: () => void
): (() => void) => {
    let unsubscribe: Unsubscribe | null = null;
    const INTERBAL = 1000 * 60 * 10;
    const unsubscribeAndClear = () => {
        if(unsubscribe){
            unsubscribe();
            unsubscribe = null;
        }
    }
    let id = setTimeout(unsubscribeAndClear,INTERBAL);
    const restart = () => {
        clearTimeout(id);
        id = setTimeout(unsubscribeAndClear,INTERBAL);
    }
    const call = () => onSnapshot(ref,(data) => {
        restart();
        callback(data);
    },onError)
    unsubscribe = call();
   
    const onMouseMove = () => {
        if(unsubscribe === null){
            unsubscribe = call();
        }
        clearTimeout(id);
        id = setTimeout(unsubscribeAndClear,INTERBAL);
    }
    window.addEventListener("mousemove",onMouseMove)
    return () => {
        clearTimeout(id);
        unsubscribeAndClear()
        window.removeEventListener("mousemove",onMouseMove)
    }
}


const syncMembers = (roomId: string,callback: (res: string) => void,onError: () => void): () => void => {
    const memberCol = collection(doc(roomCollection,roomId),"members");
    return onSnapshotWhenActive(
        memberCol,
        (res) => { callback(JSON.stringify(res.docs.map(doc => ({id: doc.id,...doc.data()}))))},
        onError
    )
}

const addMembers = (roomId: string,name: string,onComplete: () => void,onError: () => void): string => {
    const memberCol = collection(doc(roomCollection,roomId),"members");
    const docRef = doc(memberCol);
    setDoc(docRef, {name}).then(onComplete).catch(onError);
    return docRef.id;
}

const addRoom = (onComplete: (id: string) => void): void => {
    addDoc(roomCollection,{}).then((res) => {
        onComplete(res.id);
    })
}

//@ts-expect-error
window._wasm_js_bridge = {
    addMembers,
    syncMembers,
    addRoom
}
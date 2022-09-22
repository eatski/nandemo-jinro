import { initializeFirestore, doc, collection,onSnapshot, Unsubscribe, setDoc,getDocs } from "@firebase/firestore"
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

type onSnapshot = typeof onSnapshot;

const onSnapshotWhenActive: onSnapshot = (
    ref,
    callback,
    ...args
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
    },...args)
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

const syncCollection = (path: string,callback: (res: string) => void, onError: () => void): () => void => {
    const col = collection(store,path);
    return onSnapshotWhenActive(
        col,
        (res) => { callback(JSON.stringify(res.docs.map(doc => ({id: doc.id,...doc.data()}))))},
        onError
    )
}

const getCollection = (path: string,onComplete: (res: string) => void, onError: () => void) => {
    const col = collection(store,path);
    getDocs(col).then((res) => onComplete(JSON.stringify(res.docs.map(doc => ({id: doc.id,...doc.data()}))))).catch(onError);
}

const addDocument = (path: string, data: string,onComplete: (id: string) => void,onError: () => void): string => {
    const col = collection(store,path);
    const docRef = doc(col);
    setDoc(docRef, JSON.parse(data)).then(() => onComplete(docRef.id)).catch(onError);
    return docRef.id;
}

//@ts-expect-error
window._wasm_js_bridge = {
    syncCollection,
    addDocument,
    getCollection
}
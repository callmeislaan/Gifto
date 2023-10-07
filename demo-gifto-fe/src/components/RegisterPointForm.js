import './RegisterForm.css';
import InputBanner from './InputBanner';
import InputRow from './InputRow';
import React, { useEffect, useState } from 'react';
import ButtonRow from './ButtonRow';
import { useSubstrateState } from '../substrate-lib'
import { bnFromHex } from '@polkadot/util';
import { web3FromSource } from '@polkadot/extension-dapp'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import SelectRow from './SelectRow';

export default function InputForm() {
    const [symbol, setSymbol] = useState("");
    const [name, setName] = useState("");
    const [avatar, setAvatar] = useState("");
    const [description, setDescription] = useState("");
    const { api, currentAccount } = useSubstrateState();
    const [create, setCreate] = useState("Create");
    const [unsub, setUnsub] = useState(null);
    const [manager, setManager] = useState([]);

    function handleCancel(e) {
        e.preventDefault();
    }

    function handleReset(e) {
        e.preventDefault();
        setSymbol("");
        setName("");
        setAvatar("");
        setDescription("");
        setSelected(null);
    }


    const txResHandler = ({ events = [], status, txHash }) => {
        if (status.isFinalized) {
            setCreate("Create");
            toast(`❤️️ Transaction successful!`);
            unsub()
            setUnsub(null)
            return;
        }

        events.forEach(({ _, event: { data, method, section } }) => {
            if ((section + ":" + method) === 'system:ExtrinsicFailed') {
                // extract the data for this event
                const [dispatchError, dispatchInfo] = data;
                console.log(`dispatchinfo: ${dispatchInfo}`)
                let errorInfo;

                // decode the error
                if (dispatchError.isModule) {
                    // for module errors, we have the section indexed, lookup
                    // (For specific known errors, we can also do a check against the
                    // api.errors.<module>.<ErrorName>.is(dispatchError.asModule) guard)
                    const mod = dispatchError.asModule
                    const error = api.registry.findMetaError(
                        new Uint8Array([mod.index.toNumber(), bnFromHex(mod.error.toHex().slice(0, 4)).toNumber()])
                    )
                    let message = `${error.section}.${error.name}${Array.isArray(error.docs) ? `(${error.docs.join('')})` : error.docs || ''
                        }`

                    errorInfo = `${message}`;
                    console.log(`Error-info::${JSON.stringify(error)}`)
                } else {
                    // Other, CannotLookup, BadOrigin, no extra info
                    errorInfo = dispatchError.toString();
                }
                setCreate("Create");
                toast(`😞 Transaction Failed! ${section}.${method}::${errorInfo}`);
                unsub()
                setUnsub(null)
                return;
            } else if (section + ":" + method === 'system:ExtrinsicSuccess') {
                return;
            }
        });
    }

    const txErrHandler = err => {
        setCreate("Create");
        unsub()
        setUnsub(null)
        toast(`😞 Transaction Failed: ${err.toString()}`);
    }


    const getFromAcct = async () => {
        const {
            address,
            meta: { source, isInjected },
        } = currentAccount

        if (!isInjected) {
            return [currentAccount]
        }

        // currentAccount is injected from polkadot-JS extension, need to return the addr and signer object.
        // ref: https://polkadot.js.org/docs/extension/cookbook#sign-and-send-a-transaction
        const injector = await web3FromSource(source)
        return [address, { signer: injector.signer }]
    }


    const handleCreate = async (e) => {
        e.preventDefault();
        if (create !== "Create") {
            return;
        }
        setCreate("Creating");

        const txExecute = await api.tx.points.createNewPoint(symbol, selected.value, name, avatar, description)

        const fromAcct = await getFromAcct()
        // transformed can be empty parameters

        const unsubHandle = await txExecute
            .signAndSend(...fromAcct, txResHandler)
            .catch(txErrHandler)
        setUnsub(() => unsubHandle)
    }

    const [brands, setBrands] = useState([]);
    const [selected, setSelected] = useState(null);
    useEffect(async () => {
        setSelected(null);
        let initBrand = [];
        console.log(brands);
        const datas = await api.query.brands.brandOwner(currentAccount.address);
        datas.value.forEach(async (hash) => {
            const brand = await api.query.brands.brands(hash.toString());
            initBrand.push({
                value: hash,
                label: brand.toHuman().symbol
            });
        });
        setBrands(initBrand);
    }, [currentAccount]);


    return (
        <>
            <form className='register-form' onSubmit={handleCreate}>
                <table>
                    <tr>
                        <InputBanner value="Register Point" />
                    </tr>
                    <tr>
                        <InputRow name="Symbol" value={symbol} onChange={setSymbol} />
                    </tr>
                    <tr>
                        <InputRow name="Name" value={name} onChange={setName} />
                    </tr>
                    <tr>
                        <InputRow name="Avatar" value={avatar} onChange={setAvatar} />
                    </tr>
                    <tr>
                        <InputRow name="Description" value={description} onChange={setDescription} />
                    </tr>
                    <tr>
                        <SelectRow name="Brand" options={brands} value={selected} onChange={setSelected} />
                    </tr>
                    <tr>
                        <ButtonRow value1="Cancel" value2="Reset" value3={create}
                            handle1={handleCancel} handle2={handleReset} handle3={handleCreate} />
                    </tr>
                </table>
            </form>
            <ToastContainer
                position="top-right"
                autoClose={5000}
                hideProgressBar={false}
                newestOnTop={false}
                closeOnClick
                rtl={false}
                pauseOnFocusLoss
                draggable
                pauseOnHover
                theme="light"
            />
        </>
    )
}
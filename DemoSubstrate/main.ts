import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import '@polkadot/api-augment';
import { KeyringPair } from "@polkadot/keyring/types";
import type { FrameSystemAccountInfo } from "@polkadot/types/lookup";

const WEB_SOCKET = 'ws://127.0.0.1:9944';
const connect = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    return api;
}

const getFreeBalance = async (api: ApiPromise, address: string) => {

    const { data: { free, }, }: FrameSystemAccountInfo = await api.query.system.account(address);
    return free;
}

const transfer = async (api: ApiPromise, alice: KeyringPair, bob: string, amount: number) => {
    await api.tx.balances.transferAllowDeath(bob, amount)
        .signAndSend(alice, res => {
            console.log(`Ts status: ${res.status}`)
        });
}

const substrate = async (api: ApiPromise) => {
    // 定义要订阅的事件和存储项
    // const eventName = 'someThing'; // 替换为你的事件名称
    // const storageKey = 'YourStorageKey'; // 替换为你的存储项键
    // 订阅事件
    await api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            console.log('index:', event.index.toHuman());
            console.log('data:', event.data.toHuman());

            // if (event.section === 'template' && event.method === eventName) {
            //     console.log('Received event:', event);
            // }
        });
    });
    // 订阅存储项更新
    // await api.query[storageKey].subscribe((value) => {
    //     console.log('Received storage update:', value);
    // });
}

const main = async () => {
    const api = await connect();
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    // const bob_balance = await getFreeBalance(api, bob.address);
    // console.log('bob_balance is', bob_balance.toHuman());
    // await transfer(api, alice, bob.address, 10 ** 10 + 1);
    await substrate(api);
    await new Promise(resolve => setTimeout(resolve, 50000));
    // const bob_balance_agter_transfer = await getFreeBalance(api, bob.address);
    // console.log('bob_balance_agter_transfer is', bob_balance_agter_transfer.toHuman());

    // console.log('deposit is', free.toHuman());
    console.log('main function');
}

main()
    .then(() => {
        console.log('exits with success');
        process.exit(0);
    })
    .catch(err => {
        console.log('error is', err);
        process.exit(1);
    })
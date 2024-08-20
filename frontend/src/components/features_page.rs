use yew::prelude::*;

#[function_component(FeaturesPage)]
pub fn features_page() -> Html {
    html! {
        <div>
            <h1>{"Features"}</h1>
            <p>{"Explore the cutting-edge features that make Foxtrot the most advanced blockchain."}</p>

            <section>
                <h2>{"Combining Winterfell zk-STARKs with Schnorrkel"}</h2>
                <p>
                    {"Combining Winterfell zk-STARKs with Schnorrkel creates a powerful, next-generation cryptographic toolkit that brings together the best of both worlds: scalability and privacy from zk-STARKs with efficient, secure digital signatures from Schnorrkel."}
                </p>
            </section>

            <section>
                <h3>{"1. Unparalleled Scalability with Zero-Knowledge Proofs"}</h3>
                <p>
                    {"zk-STARKs (Zero-Knowledge Scalable Transparent ARguments of Knowledge) are renowned for enabling zero-knowledge proofs that are both scalable and transparent. They allow you to prove the validity of computations without revealing the underlying data, and they do so without needing a trusted setup."}
                </p>
                <p>
                    {"When combined with Schnorrkel, you can create highly scalable and private applications where not only are transactions and data verifiable without exposing sensitive information, but they are also efficiently signed and verified using Schnorr signatures."}
                </p>
                <p>
                    {"Impact: This enables massive scalability for applications like blockchains or decentralized systems, where both computational integrity and privacy are critical. You can achieve high transaction throughput while ensuring that each transaction is private and authenticated."}
                </p>
            </section>

            <section>
                <h3>{"2. Fortified Security and Decentralization"}</h3>
                <p>
                    {"Schnorrkel's multi-signature capabilities can be integrated with Winterfell's zk-STARKs to enable decentralized systems where multiple parties can collaborate on validating a zk-STARK proof. This multi-party validation ensures that the system remains secure and trustless, with no single point of failure."}
                </p>
                <p>
                    {"Winterfell zk-STARKs provide cryptographic proofs that are resistant to quantum attacks, ensuring long-term security. Coupling this with Schnorrkel's secure, compact signatures fortifies the system against a wide array of attacks, including those from future quantum computers."}
                </p>
                <p>
                    {"Impact: The combination offers a high level of security and decentralization, making it ideal for critical applications such as decentralized finance (DeFi), where the stakes are incredibly high."}
                </p>
            </section>

            <section>
                <h3>{"3. Privacy-Preserving Multi-Signature Schemes"}</h3>
                <p>
                    {"Winterfell zk-STARKs allow you to generate proofs of complex computations or transactions without revealing the inputs. When combined with Schnorrkel’s multi-signature scheme (MuSig), you can build systems where multiple parties can sign off on a zk-STARK proof collectively, all while keeping their identities and inputs private."}
                </p>
                <p>
                    {"Imagine a private, multi-signature transaction on a blockchain, where the transaction details are obscured by zk-STARKs, and the transaction is collectively signed by multiple validators using Schnorrkel. The result is a highly secure, private, and verifiable transaction that scales efficiently."}
                </p>
                <p>
                    {"Impact: This is incredibly useful in applications where privacy is paramount, such as confidential financial transactions, secure voting systems, or private data sharing among multiple parties."}
                </p>
            </section>

            <section>
                <h3>{"4. Lightning-Fast Verification for Decentralized Applications"}</h3>
                <p>
                    {"Winterfell’s zk-STARKs are designed for fast verification, which can be crucial for maintaining the performance of decentralized applications (dApps). When you pair this with Schnorrkel's efficient signature verification, you get a system where both the proof of computational integrity and the authentication process are lightning-fast."}
                </p>
                <p>
                    {"This combination enables the creation of dApps and blockchain systems that can handle large volumes of transactions or computations quickly, without sacrificing security or privacy."}
                </p>
                <p>
                    {"Impact: Users experience fast transaction times and low latency, even as the system scales. This is critical for applications in gaming, DeFi, and any other high-throughput environment."}
                </p>
            </section>

            <section>
                <h3>{"5. Next-Gen Decentralized Identity and Access Control"}</h3>
                <p>
                    {"By combining Schnorrkel with Winterfell zk-STARKs, you can create a decentralized identity system where users prove their credentials or permissions without revealing their actual identity. Schnorrkel’s signatures can be used to sign these zk-STARK proofs, ensuring that only authorized users can generate valid proofs."}
                </p>
                <p>
                    {"This is particularly useful in decentralized access control systems, where users need to authenticate themselves to access certain resources or services without revealing sensitive information."}
                </p>
                <p>
                    {"Impact: You get a highly secure, privacy-preserving identity system that can be integrated into a wide range of applications, from secure file sharing to decentralized social networks."}
                </p>
            </section>
        </div>
    }
}

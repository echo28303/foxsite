use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <>
            <section class="about-section">
                <h1 class="about-header">{ "About Foxtrot Blockchain" }</h1>
                <h3 class="about-subheading">{ "Unlock Unmatched Security and Efficiency with Schnorrkel: The Future of Cryptography for Decentralized Systems"}</h3>
                <p class="about-content">
                    { "Discover how the schnorrkel cryptographic library is revolutionizing decentralized systems with advanced features like multi-signatures, aggregate signatures, and privacy-preserving transactions. Perfect for blockchain developers and security-conscious engineers, schnorrkel offers a cutting-edge solution for building scalable, secure, and efficient applications." }
                </p>
            </section>

            <section class="about-section">
                <h3 class="about-subheading">{ "Transform Decentralized Governance with Multi-Signatures" }</h3>
                <p class="about-content">
                    { "Utilize schnorrkel to implement scalable multi-signature schemes in decentralized autonomous organizations (DAOs). Ensure secure, collective decision-making with a single, compact signature that requires agreement from multiple stakeholders. Perfect for DAOs, blockchain governance, and decentralized voting systems." }
                </p>
            </section>

            <section class="about-section">
                <h3 class="about-subheading">{ "Enable Privacy-Preserving Transactions" }</h3>
                <p class="about-content">
                    { "Combine the power of schnorrkel with zero-knowledge proofs to create secure, anonymous transactions. Whether for privacy-focused cryptocurrencies or confidential smart contracts, schnorrkel ensures that transactions are both verifiable and private, without compromising on efficiency." }
                </p>
            </section>

            <section class="about-section">
                <h3 class="about-subheading">{ "Achieve Lightning-Fast Blockchain Consensus" }</h3>
                <p class="about-content">
                    { "Implement a highly scalable blockchain consensus mechanism using schnorrkel. Reduce bandwidth and storage requirements while achieving faster block times, making your blockchain competitive with centralized systems without sacrificing decentralization." }
                </p>
            </section>

            <section class="about-section">
                <h4 class="about-subheading">{ "Ready to take your decentralized application to the next level? Explore the full potential of schnorrkel today and start building secure, scalable, and privacy-preserving solutions." }</h4>
            </section>

            
        </>
    }
}

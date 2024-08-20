use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
        <section class="hero">
            <div class="hero-content">
                <h1>{ "Welcome to Foxtrot Blockchain" }</h1>
                <p>{ "Unlock Unmatched Security and Efficiency with Schnorrkel: The Future of Cryptography for Decentralized Systems" }</p>
                <a href="#" class="btn-primary">{ "Get Started" }</a>
            </div>
        </section>

        <section class="container">
            <h2>{ "Why Choose Foxtrot?" }</h2>
            <div class="row">
                <div class="col-4">
                    <div class="card">
                        <div class="card-header">{ "Cutting-Edge Cryptography" }</div>
                        <div class="card-content">
                            { "You're leveraging two of the most advanced cryptographic technologies available today, ensuring your system is ahead of the curve." }
                        </div>
                    </div>
                </div>
                <div class="col-4">
                    <div class="card">
                        <div class="card-header">{ "Unmatched Privacy and Security" }</div>
                        <div class="card-content">
                            { "Combining zk-STARKs with Schnorr signatures offers a level of privacy and security that is difficult to match, especially in decentralized environments." }
                        </div>
                    </div>
                </div>
                <div class="col-4">
                    <div class="card">
                        <div class="card-header">{ "Scalability and Efficiency" }</div>
                        <div class="card-content">
                            { "You get the best of both worlds—scalable, fast verification from zk-STARKs and efficient, compact signatures from Schnorrkel, enabling systems that can handle high volumes of transactions or data." }
                        </div>
                    </div>
                </div>
                <div class="col-4">
                    <div class="card">
                        <div class="card-header">{ "Versatile Applications" }</div>
                        <div class="card-content">
                            { "This combination is not just theoretical; it has practical, real-world applications in areas like DeFi, decentralized identity, and secure, private communication systems." }
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="callout">
            <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse imperdiet urna nec nisi malesuada." }</p>
        </section>
        </>
    }
}

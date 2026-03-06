class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.59"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.59"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.59/six7-0.7.59-arm64-apple-darwin.tar.gz"
      sha256 "755440ead3fdb8d1d6581ce0b3baa98e745d052a90b0c3c883dd156e624f707b"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end

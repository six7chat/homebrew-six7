class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.37"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.37"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.37/six7-0.7.37-arm64-apple-darwin.tar.gz"
      sha256 "7c99a39aeffef9e6988ddc69627244211d44f4653d73153635eab6a8ab3f1613"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end

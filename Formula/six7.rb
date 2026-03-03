class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.51"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.51"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.51/six7-0.7.51-arm64-apple-darwin.tar.gz"
      sha256 "e8d5d8d032107f4f1a970c0fbe738a99f2e089b439da88fa6100a35f4c2f1436"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end

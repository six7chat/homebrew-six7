class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.8"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.8"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.8/six7-0.7.8-arm64-apple-darwin.tar.gz"
      sha256 "6241a6703923d5f37dc58f69357e128a810744a5e12bb76fab2f59d399f85e26"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end

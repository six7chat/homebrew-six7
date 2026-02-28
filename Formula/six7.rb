class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.45"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.45"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.45/six7-0.7.45-arm64-apple-darwin.tar.gz"
      sha256 "c2235080d911c0c40ed26013f4d85789ccc975887d755caa0e8b64881e1830cc"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
